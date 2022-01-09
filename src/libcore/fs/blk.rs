// src/libcore/fs/blk.rs
//
// Basic implementation of blocks for working with filesystems.

/*
	IMPORTS
*/

use core::convert::TryInto;

use crate::{serprintln, libcore::fs::{blkdev::BlkDevIO, bmapblk::BMapBlk}};


const DATAOFFSET: usize = 4;


// Block struct
#[derive(Clone)]
pub struct Blk
{
	address: u32,
	buffer: [u8; crate::libcore::fs::ata::BLKSIZE],
}


// Linked-block struct
pub struct LinkBlk
{
	blk: Blk
}


// Implementation of the Blk struct
impl Blk
{
	// Address
	pub fn address(&self) -> u32
	{
		self.address
	}

	// Allocate
	pub fn alloc() -> Option<Self>
	{
		match BMapBlk::next_free_address()
		{
			None =>
			{
				None
			}
			Some(address) =>
			{
				BMapBlk::alloc(address);
				let mut blk = Blk::read(address);

				for i in 0..crate::libcore::fs::ata::BLKSIZE
				{
					blk.buffer[i] = 0;
				}
				blk.write();

				Some(blk)
			}
		}
	}

	// Data
	pub fn data(&self) -> &[u8]
	{
		&self.buffer[..]
	}

	// Data (mutable)
	pub fn datamut(&mut self) -> &mut [u8]
	{
		&mut self.buffer[..]
	}

	// New
	pub fn new(address: u32) -> Self
	{
		let buffer = [0; crate::libcore::fs::ata::BLKSIZE];
		Self
		{
			address,
			buffer,
		}
	}

	// Read
	pub fn read(address: u32) -> Self
	{
		let mut buffer = [0; crate::libcore::fs::ata::BLKSIZE];
		if let Some(ref blkdev) = *crate::libcore::fs::blkdev::BLKDEV.lock()
		{
			if blkdev.read(address, &mut buffer).is_err()
			{
				serprintln!("[ERR] COULD NOT READ LIBFS BLOCK {:#x}", address);
			}
		}
		Self
		{
			address,
			buffer,
		}
	}

	// Write
	pub fn write(&self)
	{
		if let Some(ref mut blkdev) = *crate::libcore::fs::blkdev::BLKDEV.lock()
		{
			if blkdev.write(self.address, &self.buffer).is_err()
			{
				serprintln!("[ERR] COULD NOT WRITE BLOCK: {:#x}", self.address);
			}
		}
	}
}


// Implementation of the LinkBlk struct
impl LinkBlk
{
	// Address
	pub fn address(&self) -> u32
	{
		self.blk.address()
	}

	// Allocate
	pub fn alloc() -> Option<Self>
	{
		Blk::alloc().map(|blk| Self
		{
			blk
		})
	}


	// Allocate next
	pub fn alloc_next(&mut self) -> Option<Self>
	{
		let newblk = LinkBlk::alloc()?;
		self.set_next_address(newblk.address());
		self.write();
		Some(newblk)
	}


	// Data
	pub fn data(&self) -> &[u8]
	{
		&self.blk.buffer[DATAOFFSET..crate::libcore::fs::ata::BLKSIZE]
	}


	// Data (mutable)
	pub fn datamut(&mut self) -> &mut [u8]
	{
		&mut self.blk.buffer[DATAOFFSET..crate::libcore::fs::ata::BLKSIZE]
	}


	// Length
	pub fn len(&self) -> usize
	{
		crate::libcore::fs::ata::BLKSIZE - DATAOFFSET
	}


	// New
	pub fn new(address: u32) -> Self
	{
		Self
		{
			blk: Blk::new(address)
		}
	}


	// Next
	pub fn next(&self) -> Option<Self>
	{
		let address = u32::from_be_bytes(self.blk.buffer[0..4].try_into().unwrap());

		if address == 0
		{
			None
		}
		else
		{
			Some(Self::read(address))
		}
	}


	// Read
	pub fn read(address: u32) -> Self
	{
		Self
		{
			blk: Blk::read(address)
		}
	}


	// Set next address
	pub fn set_next_address(&mut self, address: u32)
	{
		self.blk.buffer[0..4].clone_from_slice(&address.to_be_bytes());
	}


	// Write
	pub fn write(&self)
	{
		self.blk.write()
	}
}
