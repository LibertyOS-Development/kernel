// src/fs/blkdev.rs
//
// Basic functionality for working with block-devices.

use alloc::vec;
use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static!
{
	pub static ref BLKDEV: Mutex<Option<BLKDEV>> = Mutex::new(None);
}

pub enum BLKDEV
{
	ATA(ATA_BLKDEV),
	MEM(MEM_BLKDEV),
}

pub trait BLKDEV_IO
{
	fn blkcount(&self) -> usize;
	fn blksize(&self) -> usize;
	fn read(&self, address: u32, buf: &mut [u8]);
	fn write(&mut self, address: u32, buf: &[u8]);
}


impl BLKDEV_IO for BLKDEV
{
	fn blkcount(&self) -> usize
	{
		match self
		{
			BLKDEV::ATA(dev) => dev.blkcount() as usize,
			BLKDEV::MEM(dev) => dev.blkcount() as usize,
		}
	}

	fn blksize(&self) -> usize
	{
		match self
		{
			BLKDEV::ATA(dev) => dev.blksize() as usize,
			BLKDEV::MEM(dev) => dev.blksize() as usize,
		}
	}

	fn read(&self, address: u32, buf: &mut [u8])
	{
		match self
		{
			BLKDEV::ATA(dev) => dev.read(address, buf),
			BLKDEV::MEM(dev) => dev.read(address, buf),
		}
	}

	fn write(&mut self, address: u32, buf: &[u8])
	{
		match self
		{
			BLKDEV::ATA(dev) => dev.write(address, buf),
			BLKDEV::MEM(dev) => dev.write(address, buf),
		}
	}
}

pub struct MEM_BLKDEV
{
	dev: Vec<[u8; super::BLKSIZE]>,
}

impl MEM_BLKDEV
{
	pub fn new(len: usize) -> Self
	{
		let dev = vec![[0; super::BLKSIZE]; len];
		Self
		{
			dev
		}
	}
}

impl BLKDEV_IO for MEM_BLKDEV
{
	fn blkcount(&self) -> usize
	{
		self.dev.len()
	}

	fn blksize(&self) -> usize
	{
		super::BLKSIZE
	}

	fn read(&self, blkidx: u32, buf: &mut [u8])
	{
		buf[..].clone_from_slice(&self.dev[blkidx as usize][..]);
	}

	fn write(&mut self, blkidx: u32, buf: &[u8])
	{
		self.dev[blkidx as usize][..].clone_from_slice(&buf[..]);
	}
}


pub fn MOUNT_MEM()
{
	let mem = 
