// src/libcore/fs/directory.rs
//
// Basic directory functionality for working with filesystems.

/*
	IMPORTS
*/

use alloc::string::String;
use core::convert::From;

use crate::libcore::fs::{FileIO, sblk::SBlk};

// Basic directory struct
#[derive(Debug, Clone, Copy)]
pub struct Directory
{
	address: u32,
}


// Implementation of the Directory struct
impl Directory
{
	// Address
	pub fn address(&self) -> u32
	{
		self.address
	}

	// Root
	pub fn root() -> Self
	{
		Self
		{
			address: SBlk::read().data_area()
		}
	}
}


// Implementation of the FileIO trait for the Directory struct
impl FileIO for Directory
{
	// Read
	fn read(&mut self, _buffer: &mut [u8]) -> Result<usize, ()>
	{
		Err(())
	}

	// Write
	fn write(&mut self, _buffer: &[u8]) -> Result<usize, ()>
	{
		Err(())
	}
}
