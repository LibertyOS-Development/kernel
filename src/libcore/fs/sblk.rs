// src/libcore/fs/sblk.rs
//
// Basic super-block functionality for working with filesystems.

/*
	IMPORTS
*/

use core::convert::TryInto;
use crate::{KSIZE, serprint, libcore::fs::{VERSION, blk::Blk, blkdev::BlkDevIO}};

const SBLK_ADDR: u32 = (KSIZE / crate::libcore::fs::ata::BLKSIZE) as u32;
const SIG: &[u8; 5] = b"LIBFS";


// Basic super-block struct
pub struct SBlk
{
	sig: &'static[u8; 5],
	vers: u8,
	blksize: u32,
	pub blkcount: u32,
	pub alloc_count: u32,
}


// Implementation of the Sblk struct
impl SBlk
{
	// Block count
	pub fn blkcount(&self) -> u32
	{
		self.blkcount
	}

	// Block size
	pub fn blksize(&self) -> u32
	{
		self.blksize
	}

	// Bitmap area
	pub fn bmap_area(&self) -> u32
	{
		SBLK_ADDR + 2
	}

	// Check ATA
	pub fn checkata(bus: u8, disk: u8) -> bool
	{
		let mut buffer = [0u8; crate::libcore::fs::ata::BLKSIZE];
		if crate::libcore::fs::ata::read(bus, disk, SBLK_ADDR, &mut buffer).is_err()
		{
			return false;
		}
		&buffer[0..8] == SIG
	}

	// Data area
	pub fn data_area(&self) -> u32
	{
		let bmapsize = crate::libcore::fs::bmapblk::BMAPSIZE as u32;
		let total = self.blkcount;
		let offset = self.bmap_area();
		let rem = (total - offset) * bmapsize / (bmapsize + 1);
		self.bmap_area() + rem / bmapsize
	}

	// Write
	pub fn new() -> Option<Self>
	{
		if let Some(ref dev) = *crate::libcore::fs::blkdev::BLKDEV.lock()
		{
			Some(Self
			{
				sig: SIG,
				vers: VERSION,
				blksize: dev.blksize() as u32,
				blkcount: dev.blkcount() as u32,
				alloc_count: 0,
			})
		}
		else
		{
			None
		}
	}

	// Read
	pub fn read() -> Self
	{
		let blk = Blk::read(SBLK_ADDR);
		let data = blk.data();
		// TODO: Add debugging message
		// serprint!(&data[0..8], SIG);

		Self
		{
			sig: SIG,
			vers: data[5],
			blksize: 2 << (8 + data[9] as u32),
			blkcount: u32::from_be_bytes(data[10..14].try_into().unwrap()),
			alloc_count: u32::from_be_bytes(data[14..18].try_into().unwrap()),
		}
	}

	// Write
	pub fn write(&self)
	{
		let mut blk = Blk::new(SBLK_ADDR);
		let data = blk.datamut();

		data[0..5].clone_from_slice(self.sig);
		data[5] = self.vers;

		let size = self.blksize;
		debug_assert!(size >= 512);
		debug_assert!(size.is_power_of_two());
		data[9] = (size.trailing_zeros() as u8) - 9;
		data[10..14].clone_from_slice(&self.blkcount.to_be_bytes());
		data[14..18].clone_from_slice(&self.alloc_count.to_be_bytes());

		blk.write();
	}
}


// Increase allocation count by one (1)
pub fn alloc_count_up()
{
	let mut sb = SBlk::read();
	sb.alloc_count += 1;
	sb.write();
}


// Decrease allocation count by one (1)
pub fn alloc_count_down()
{
	let mut sb = SBlk::read();
	sb.alloc_count -= 1;
	sb.write();
}
