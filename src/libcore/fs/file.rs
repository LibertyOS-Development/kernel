// src/libcore/fs/file.rs
//
// Basic file functionality for working with filesystems.

/*
	IMPORTS
*/

use alloc::{string::{String, ToString}, vec};
use core::convert::From;
use crate::libcore::fs::{blk::LinkBlk, directory::Directory, directory_entry::DirectoryEntry, dname, FileIO, fname, rpath};

// SeekFrom enumeration
pub enum SeekFrom
{
	Current(i32),
	End(i32),
	Start(u32),
}


// Basic file structure
#[derive(Debug, Clone)]
pub struct File
{
	address: u32,
	directory: Directory,
	name: String,
	offset: u32,
	size: u32,
}


// Implementation of the File struct
impl File
{
	// Address
	pub fn address(&self) -> u32
	{
		self.address
	}


	// Create
	pub fn create(pname: &str) -> Option<Self>
	{
		let pname = rpath(pname);
		let dname = dname(&pname);
		let fname = fname(&pname);

		if let Some(directory) = Directory::open(dname)
		{
			if let Some(directory_entry) = directory.new_file(fname)
			{
				return Some(directory_entry.into());
			}
		}
		None
	}


	// Delete
	pub fn del(pname: &str) -> Result<(), ()>
	{
		let pname = rpath(pname);
		let dname = dname(&pname);
		let fname = fname(&pname);
		if let Some(mut directory) = Directory::open(dname)
		{
			directory.item_del(fname)
		}
		else
		{
			Err(())
		}
	}


	// Name
	pub fn name(&self) -> String
	{
		self.name.clone()
	}


	// Open
	pub fn open(pname: &str) -> Option<Self>
	{
		let pname = rpath(pname);
		let dname = dname(&pname);
		let fname = fname(&pname);

		if let Some(directory) = Directory::open(dname)
		{
			if let Some(directory_entry) = directory.find(fname)
			{
				if directory_entry.isfile()
				{
					return Some(directory_entry.into());
				}
			}
		}
		None
	}


	// Read to string
	pub fn read_to_str(&mut self) -> String
	{
		let mut buffer = vec![0; self.size()];
		if let Ok(bytes) = self.read(&mut buffer)
		{
			buffer.resize(bytes, 0);
		}

		String::from_utf8_lossy(&buffer).to_string()
	}


	// Seek
	pub fn seek(&mut self, pos: SeekFrom) -> Result<u32, ()>
	{
		let offset = match pos
		{
			SeekFrom::Start(i) => i as i32,
			SeekFrom::Current(i) => i + self.offset as i32,
			SeekFrom::End(i) => i + self.size as i32 -1,
		};

		if offset < 0 || offset > self.size as i32
		{
			return Err(())
		}

		self.offset = offset as u32;

		Ok(self.offset)
	}


	// Size
	pub fn size(&self) -> usize
	{
		self.size as usize
	}
}


// Implementation of the DirectoryEntry trait for File
impl From<DirectoryEntry> for File
{
	// From
	fn from(entry: DirectoryEntry) -> Self
	{
		Self
		{
			name: entry.name(),
			address: entry.address(),
			size: entry.size(),
			directory: entry.directory(),
			offset: 0,
		}
	}
}


// Implementation of the FileIO trait for the File struct
impl FileIO for File
{
	// Read
	fn read(&mut self, buffer: &mut [u8]) -> Result<usize, ()>
	{
		let bufferlen = buffer.len();
		let mut address = self.address;
		let mut bytes = 0;
		let mut pos = 0;

		loop
		{
			let blk = LinkBlk::read(address);
			let data = blk.data();
			let datalen = data.len();
			for i in 0..datalen
			{
				if pos == self.offset
				{
					if bytes == bufferlen || pos as usize == self.size()
					{
						return Ok(bytes);
					}
					buffer[bytes] = data[i];
					bytes += 1;
					self.offset += 1;
				}
				pos += 1;
			}
			match blk.next()
			{
				Some(nextblk) => address = nextblk.address(),
				None => return Ok(bytes),
			}
		}
	}


	// Write
	fn write(&mut self, buffer: &[u8]) -> Result<usize, ()>
	{
		let bufferlen = buffer.len();
		let mut address = self.address;
		let mut bytes = 0;
		let mut pos = 0;
		while bytes < bufferlen
		{
			let mut blk = LinkBlk::read(address);
			let data = blk.datamut();
			let datalen = data.len();
			for i in 0..datalen
			{
				if pos == self.offset
				{
					if bytes == bufferlen
					{
						break;
					}
					data[i] = buffer[bytes];
					bytes += 1;
					self.offset += 1;
				}
				pos += 1;
			}

			address = match blk.next()
			{
				Some(nextblk) =>
				{
					if bytes < bufferlen
					{
						nextblk.address()
					}
					else
					{
						0
					}
				}

				None =>
				{
					if bytes < bufferlen
					{
						match LinkBlk::alloc()
						{
							Some(nextblk) => nextblk.address(),
							None => return Err(()),
						}
					}
					else
					{
						0
					}
				}
			};

			blk.set_next_address(address);
			blk.write();
		}

		self.size = self.offset;
		self.directory.item_update(&self.name, self.size);
		Ok(bytes)
	}
}

