// src/libcore/fs/directory.rs
//
// Basic directory functionality for working with filesystems.

/*
	IMPORTS
*/

use alloc::string::String;
use core::convert::From;

use crate::libcore::fs::{blk::LinkBlk, bmapblk::BMapBlk, directory_entry::DirectoryEntry, directory_read::ReadDirectory, FileIO, FileType, sblk::SBlk, rpath};


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


	// Create new directory
	pub fn create(pname: &str) -> Option<Self>
	{
		let pname = rpath(pname);
		let dname = crate::libcore::fs::dname(&pname);
		let fname = crate::libcore::fs::fname(&pname);

		if let Some(directory) = Directory::open(dname)
		{
			if let Some(directory_entry) = directory.new_dir(fname)
			{
				return Some(directory_entry.into());
			}
		}
		None
	}


	// Find
	pub fn find(&self, name: &str) -> Option<DirectoryEntry>
	{
		for item in self.items()
		{
			if item.name() == name
			{
				return Some(item);
			}
		}
		None
	}


	// Items
	pub fn items(&self) -> ReadDirectory
	{
		ReadDirectory::from(self.clone())
	}


	// Delete item
	pub fn item_del(&mut self, name: &str) -> Result<(), ()>
	{
		let mut items = self.items();
		for item in &mut items
		{
			if item.name() == name
			{
				let i = items.blk_data_offset() - item.len();
				let data = items.blk.datamut();

				data[i + 1] = 0;
				data[i + 2] = 0;
				data[i + 3] = 0;
				data[i + 4] = 0;
				items.blk.write();

				let mut itemblk = LinkBlk::read(item.address());

				loop
				{
					BMapBlk::free(itemblk.address());
					match itemblk.next()
					{
						Some(nextblk) => itemblk = nextblk,
						None => break,
					}
				}

				return Ok(());
			}
		}
		Err(())
	}


	// Update item
	pub fn item_update(&mut self, name: &str, size: u32)
	{
		let time = crate::clock::realtime() as u64;
		let mut items = self.items();
		for item in &mut items
		{
			if item.name() == name
			{
				let i = items.blk_data_offset() - item.len();
				let data = items.blk.datamut();
				data[(i + 5)..(i + 9)].clone_from_slice(&size.to_be_bytes());
				data[(i + 9)..(i + 17)].clone_from_slice(&time.to_be_bytes());
				items.blk.write();
				break;
			}
		}
	}


	// Create a new device
	pub fn new_dev(&self, name: &str) -> Option<DirectoryEntry>
	{
		self.new_item(FileType::Dev, name)
	}


	// Create a new directory
	pub fn new_dir(&self, name: &str) -> Option<DirectoryEntry>
	{
		self.new_item(FileType::File, name)
	}


	// New file
	pub fn new_file(&self, name: &str) -> Option<DirectoryEntry>
	{
		self.new_item(crate::libcore::fs::FileType::File, name)
	}


	// Create a new item
	pub fn new_item(&self, tp: crate::libcore::fs::FileType, name: &str) -> Option<DirectoryEntry>
	{
		if self.find(name).is_some()
		{
			return None;
		}

		let mut items = self.items();
		while items.next().is_some() {}

		let rem_space = items.blk.data().len() - items.blk_data_offset();
		let itemlen = DirectoryEntry::len_null() + name.len();

		if itemlen > rem_space
		{
			match items.blk.alloc_next()
			{
				// Not enough space on the disk
				None => return None,

				Some(newblk) =>
				{
					items.blk = newblk;
					items.blk_data_offset = 0;
				},
			}
		}

		let item_blk = LinkBlk::alloc().unwrap();
		let item_type = tp as u8;
		let item_address = item_blk.address();
		let item_size = 0u32;
		let item_time = crate::clock::realtime() as u64;
		let item_name = trunc(name, u8::MAX as usize);
		let n = item_name.len();
		let i = items.blk_data_offset();
		let data = items.blk.datamut();

		data[i] = item_type;
		data[(i + 1)..(i + 5)].clone_from_slice(&item_address.to_be_bytes());
		data[(i + 5)..(i + 9)].clone_from_slice(&item_size.to_be_bytes());
		data[(i + 9)..(i + 17)].clone_from_slice(&item_time.to_be_bytes());
		data[i + 17] = n as u8;
		data[(i + 18)..(i + 18 + n)].clone_from_slice(item_name.as_bytes());

		items.blk.write();

		Some(DirectoryEntry::new(*self, tp, item_address, item_size, item_time, &item_name))
	}

	// Open
	pub fn open(pname: &str) -> Option<Self>
	{
		if !crate::libcore::fs::blkdev::mounted()
		{
			return None;
		}

		let mut directory = Directory::root();
		let pname = rpath(pname);

		if pname == "/"
		{
			return Some(directory);
		}

		for name in pname.trim_start_matches('/').split('/')
		{
			match directory.find(name)
			{
				Some(directory_entry) =>
				{
					if directory_entry.isdir()
					{
						directory = directory_entry.into()
					}
					else
					{
						return None;
					}
				},

				None =>
				{
					return None
				},
			}
		}
		Some(directory)
	}


	// Remove
	pub fn rm(pname: &str) -> Result<(), ()>
	{
		let pname = crate::libcore::fs::rpath(pname);
		let dname = crate::libcore::fs::dname(&pname);
		let fname = crate::libcore::fs::fname(&pname);

		if let Some(mut directory) = Directory::open(dname)
		{
			directory.item_del(fname)
		}
		else
		{
			Err(())
		}
	}


	// Root
	pub fn root() -> Self
	{
		Self
		{
			address: SBlk::read().data_area()
		}
	}


	// Update item
	pub fn update_item(&mut self, name: &str, size: u32)
	{
		let time = crate::clock::realtime() as u64;
		let mut items = self.items();
		for item in &mut items
		{
			if item.name() == name
			{
				let i = items.blk_data_offset() - item.len();
				let data = items.blk.datamut();

				data[(i + 5)..(i + 9)].clone_from_slice(&size.to_be_bytes());
				data[(i + 9)..(i + 17)].clone_from_slice(&time.to_be_bytes());
				items.blk.write();

				break;
			}
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


// Implementation of From<DirectoryEntry> for the Directory struct
impl From<DirectoryEntry> for Directory
{
	// From
	fn from(item: DirectoryEntry) -> Self
	{
		Self
		{
			address: item.address()
		}
	}
}


// Truncate
fn trunc(s: &str, max: usize) -> String
{
	s.char_indices().take_while(|(i, _)| *i <= max).map(|(_, c)| c).collect()
}
