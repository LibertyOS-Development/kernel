// src/libcore/fs/blkdev.rs
//
// Basic block-device functionality for working with filesystems.

/*
	IMPORTS
*/

use alloc::{vec, vec::Vec};
use lazy_static::lazy_static;
use spin::Mutex;

use crate::libcore::fs::{bmapblk::BMapBlk, directory::Directory, sblk::SBlk};


lazy_static!
{
	pub static ref BLKDEV: Mutex<Option<BlkDev>> = Mutex::new(None);
}


// Basic block-device enumeration
pub enum BlkDev
{
	MEM(MemBlkDev),
	ATA(AtaBlkDev),
}


// BlkDevIO trait
pub trait BlkDevIO
{
	fn read(&self, address: u32, buffer: &mut [u8]) -> Result<(), ()>;
	fn write(&mut self, address: u32, buffer: &[u8]) -> Result<(), ()>;
	fn blksize(&self) -> usize;
	fn blkcount(&self) -> usize;
}


// Implementation of the BlkDevIO trait for BlkDev
impl BlkDevIO for BlkDev
{
	// Block count
	fn blkcount(&self) -> usize
	{
		match self
		{
			BlkDev::MEM(dev) => dev.blkcount() as usize,
			BlkDev::ATA(dev) => dev.blkcount() as usize,
		}
	}

	// Block size
	fn blksize(&self) -> usize
	{
		match self
		{
			BlkDev::MEM(dev) => dev.blksize() as usize,
			BlkDev::ATA(dev) => dev.blksize() as usize,
		}
	}

	// Read
	fn read(&self, address: u32, buffer: &mut [u8]) -> Result<(), ()>
	{
		match self
		{
			BlkDev::MEM(dev) => dev.read(address, buffer),
			BlkDev::ATA(dev) => dev.read(address, buffer),
		}
	}


	// Write
	fn write(&mut self, address: u32, buffer: &[u8]) -> Result<(), ()>
	{
		match self
		{
			BlkDev::MEM(dev) => dev.write(address, buffer),
			BlkDev::ATA(dev) => dev.write(address, buffer),
		}
	}
}


// MemBlkDev struct
pub struct MemBlkDev
{
	device: Vec<[u8; crate::libcore::fs::ata::BLKSIZE]>,
}


// Implementation of the MemBlkDev struct
impl MemBlkDev
{
	pub fn new(len: usize) -> Self
	{
		let device = vec![[0; crate::libcore::fs::ata::BLKSIZE]; len];
		Self
		{
			device
		}
	}
}


// Implementation of the BlkDevIO for the MemBlkDev struct
impl BlkDevIO for MemBlkDev
{
	// Block count
	fn blkcount(&self) -> usize
	{
		self.device.len()
	}

	// Block size
	fn blksize(&self) -> usize
	{
		crate::libcore::fs::ata::BLKSIZE
	}

	// Read
	fn read(&self, blkidx: u32, buffer: &mut [u8]) -> Result<(), ()>
	{
		buffer[..].clone_from_slice(&self.device[blkidx as usize][..]);
		Ok(())
	}

	// Write
	fn write(&mut self, blkidx: u32, buffer: &[u8]) -> Result<(), ()>
	{
		self.device[blkidx as usize][..].clone_from_slice(buffer);
		Ok(())
	}
}


// AtaBlkDev struct
#[derive(Clone)]
pub struct AtaBlkDev
{
	device: crate::libcore::fs::ata::Drive
}


// Implementation of the AtaBlkDev struct
impl AtaBlkDev
{
	pub fn new(bus: u8, disk: u8) -> Option<Self>
	{
		crate::libcore::fs::ata::Drive::open(bus, disk).map(|device|
		{
			Self
			{
				device
			}
		})
	}
}


// Implementation of the BlkDevIO trait for the AtaBlkDev struct
impl BlkDevIO for AtaBlkDev
{
	// Block count
	fn blkcount(&self) -> usize
	{
		self.device.blkcount() as usize
	}

	// Block size
	fn blksize(&self) -> usize
	{
		self.device.blksize() as usize
	}

	// Read
	fn read(&self, blkaddr: u32, buffer: &mut [u8]) -> Result<(), ()>
	{
		crate::libcore::fs::ata::read(self.device.bus, self.device.disk, blkaddr, buffer)
	}

	// Write
	fn write(&mut self, blkaddr: u32, buffer: &[u8]) -> Result<(), ()>
	{
		crate::libcore::fs::ata::write(self.device.bus, self.device.disk, blkaddr, buffer)
	}
}


// Dismount
pub fn dismount()
{
	*BLKDEV.lock() = None;
}


// Format ATA
pub fn fmtata()
{
	if let Some(sb) = SBlk::new()
	{
		// Write sblk
		sb.write();

		// Write zeros to blkbmaps
		crate::libcore::fs::bmapblk::freeall();

		// Alloc root directory
		// TODO: Add debug info to check if drive is mounted
		let root = Directory::root();
		BMapBlk::alloc(root.address());
	}
}


// Format memory
pub fn fmtmem()
{
	// TODO: Add debug information
	if let Some(sb) = SBlk::new()
	{
		sb.write();
		let root = Directory::root();
		BMapBlk::alloc(root.address());
	}
}


// Mount ATA
pub fn mntata(bus: u8, disk: u8)
{
	*BLKDEV.lock() = AtaBlkDev::new(bus, disk).map(BlkDev::ATA);
}


// Mount memory
pub fn mntmem()
{
	// Allocate half of available memory
	let memory = crate::libcore::allocator::memsize() / 2;
	let len = memory / crate::libcore::fs::ata::BLKSIZE;
	let device = MemBlkDev::new(len);

	*BLKDEV.lock() = Some(BlkDev::MEM(device));
}


// Whether or not drive has been mounted
pub fn mounted() -> bool
{
	BLKDEV.lock().is_some()
}
