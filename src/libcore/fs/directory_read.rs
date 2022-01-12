// src/libcore/fs/directory_read.rs
//
// Read from a directory.

/*
	IMPORTS
*/

use alloc::string::String;
use core::convert::{From, TryInto};

use crate::{read_ui_func, libcore::fs::{blk::LinkBlk, directory::Directory, directory_entry::DirectoryEntry, FileType}};

// ReadDirectory struct
pub struct ReadDirectory
{
	pub directory: Directory,
	pub blk: LinkBlk,
	pub blk_data_offset: usize,
}


// Implementation of the ReadDirectory struct
impl ReadDirectory
{
	// Block address
	pub fn blk_address(&self) -> u32
	{
		self.blk.address()
	}


	// Block data offset
	pub fn blk_data_offset(&self) -> usize
	{
		self.blk_data_offset
	}

	// Read u8
	read_ui_func!(read_u8, u8);

	// Read u32
	read_ui_func!(read_u32, u32);

	// Read u64
	read_ui_func!(read_u64, u64);

	// Read UTF-8 (lossy)
	fn read_utf8_lossy(&mut self, len: usize) -> String
	{
		let data = self.blk.data();
		let a = self.blk_data_offset;
		let b = a + len;
		self.blk_data_offset = b;
		String::from_utf8_lossy(&data[a..b]).into()
	}
}

// Implementation of From<Directory> for the ReadDirectory struct
impl From<Directory> for ReadDirectory
{
	// From
	fn from(directory: Directory) -> Self
	{
		Self
		{
			directory,
			blk: LinkBlk::read(directory.address()),
			blk_data_offset: 0,
		}
	}
}


// Implementation of the Iterator trait for the ReadDirectory struct
impl Iterator for ReadDirectory
{
	type Item = DirectoryEntry;

	// Next
	fn next(&mut self) -> Option<DirectoryEntry>
	{
		loop
		{
			loop
			{
				// Backup the cursor's position
				let offset = self.blk_data_offset;

				if offset >= self.blk.len() - DirectoryEntry::len_null()
				{
					break;
				}

				let item_tp = match self.read_u8()
				{
					0 => FileType::Directory,
					1 => FileType::File,
					2 => FileType::Dev,
					_ =>
					{
						// Reverse/rewind the cursor
						self.blk_data_offset = offset;
						break;
					},
				};

				let item_address = self.read_u32();
				let item_size = self.read_u32();
				let item_time = self.read_u64();

				let n = self.read_u8() as usize;
				if n == 0 || n >= self.blk.len() - self.blk_data_offset
				{
					// Reverse/rewind the cursor
					self.blk_data_offset = offset;
					break;
				}

				let item_name = self.read_utf8_lossy(n);

				// Ignore deleted items
				if item_address == 0
				{
					continue;
				}

				return Some(DirectoryEntry::new(self.directory, item_tp, item_address, item_size, item_time, &item_name));
			}

			match self.blk.next()
			{
				Some(nextblk) =>
				{
					self.blk = nextblk;
					self.blk_data_offset = 0;
				}
				None => break,
			}
		}

		None
	}
}
