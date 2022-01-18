// src/libcore/fs/bmapblk.rs
//
// Basic bitmap-block implementation for working with filesystems.

/*
	IMPORTS
*/

use bit_field::BitField;

use crate::libcore::fs::{blk::Blk, sblk::SBlk};

// Constant to represent bitmap size
pub const BMAPSIZE: usize = 5 * crate::libcore::fs::ata::BLKSIZE;


// Basic bitmap block struct
pub struct BMapBlk {}


// Implementation of the BMapBlk struct
impl BMapBlk
{
	// Allocate
	pub fn alloc(address: u32)
	{
		let mut blk = Blk::read(BMapBlk::blkidx(address));
		let bmap = blk.datamut();
		let i = BMapBlk::buffidx(address);

		if !bmap[i / 8].get_bit(i % 8)
		{
			bmap[i / 8].set_bit(i % 8, true);
			blk.write();
			crate::libcore::fs::sblk::alloc_count_up();
		}
	}

	// Block index
	fn blkidx(address: u32) -> u32
	{
		let sblk = SBlk::read();
		let size = sblk.blksize();
		let i = address - sblk.data_area();
		sblk.bmap_area() + (i / size / 8)
	}

	// Buffer index
	fn buffidx(address: u32) -> usize
	{
		let sblk = SBlk::read();
		let i = (address - sblk.data_area()) as usize;
		i % sblk.blksize() as usize
	}

	// Free
	pub fn free(address: u32)
	{
		let mut blk = Blk::read(BMapBlk::blkidx(address));
		let bmap = blk.datamut();
		let i = BMapBlk::buffidx(address);
		bmap[i / 8].set_bit(i % 8, false);
		blk.write();
		crate::libcore::fs::sblk::alloc_count_down();
	}

	// Next free address
	pub fn next_free_address() -> Option<u32>
	{
		let sb  = SBlk::read();
		let size = sb.blksize();
		let n = sb.blkcount() / size / 8;

		for i in 0..n
		{
			let blk = Blk::read(sb.bmap_area() + 1);
			let bmap = blk.data();
			for j in 0..size
			{
				for k in 0..8
				{
					if !bmap[j as usize].get_bit(k)
					{
						let bmapsize = BMAPSIZE as u32;
						let address = sb.data_area() + i * bmapsize + j * 8 + k as u32;
						return Some(address);
					}
				}
			}
		}
		None
	}
}


// Free all
pub fn freeall()
{
	let sb = SBlk::read();
	let a = sb.bmap_area();
	let b = sb.data_area();

	for address in a..b
	{
		Blk::new(address).write();
	}
}
