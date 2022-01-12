// src/libcore/fs/mod.rs
//
// This is the mod.rs file for the libcore::fs module.


/*
	IMPORTS
*/

use alloc::{format, string::{String, ToString}, vec, vec::Vec};

//use crate::libcore::{fs::{dev::Device, directory::Directory, directory_entry::DirectoryEntry, file::File}};
//use crate::libcore::fs::directory_entry::FileInfo;
//use crate::libcore::fs::fname;

use crate::libcore::fs::bmapblk::BMAPSIZE;
use crate::libcore::fs::dev::{Device, DevType};
use crate::libcore::fs::directory::Directory;
use crate::libcore::fs::file::{File, SeekFrom};
use crate::libcore::fs::blkdev::{fmtata, fmtmem, mounted, mntata, mntmem, dismount};
use crate::libcore::fs::directory_entry::{DirectoryEntry, FileInfo};


pub mod ata;
pub mod blk;
pub mod blkdev;
pub mod bmapblk;
pub mod dev;
pub mod directory;
pub mod directory_entry;
pub mod directory_read;
pub mod file;
pub mod sblk;


pub const VERSION: u8 = 1;


// FileType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType
{
	Directory = 0,
	File = 1,
	Dev = 2,
}


// FileIO trait
pub trait FileIO
{
	fn read(&mut self, buffer: &mut [u8]) -> Result<usize, ()>;
	fn write(&mut self, buffer: &[u8]) -> Result<usize, ()>;
}


// OpenFlag enumeration
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum OpenFlag
{
	READ = 1,
	WRITE = 2,
	CREATE = 4,
	DIRECTORY = 8,
	DEVICE = 16,
}


// Resource enumeration
#[derive(Debug, Clone)]
pub enum Resource
{
	Device(Device),
	Directory(Directory),
	File(File),
}


// Implementation of the FileIO trait for the Resuource enumeration
impl FileIO for Resource
{
	// Read
	fn read(&mut self, buffer: &mut [u8]) -> Result<usize, ()>
	{
		match self
		{
			Resource::Directory(io) => io.read(buffer),
			Resource::File(io) => io.read(buffer),
			Resource::Device(io) => io.read(buffer),
		}
	}

	// Write
	fn write(&mut self, buffer: &[u8]) -> Result<usize, ()>
	{
		match self
		{
			Resource::Directory(io) => io.write(buffer),
			Resource::File(io) => io.write(buffer),
			Resource::Device(io) => io.write(buffer),
		}
	}
}

// Implementation of the OpenFlag enumeration
impl OpenFlag
{
	fn set(&self, flags: usize) -> bool
	{
		flags & (*self as usize) != 0
	}
}


// Canonicalize
pub fn canon(path: &str) -> Result<String, ()>
{
	match crate::libcore::sys::proc::env("HOME")
	{
		Some(home) =>
		{
			if path.starts_with('~')
			{
				Ok(path.replace('~', &home))
			}
			else
			{
				Ok(path.to_string())
			}
		},

		None =>
		{
			Ok(path.to_string())
		}
	}
}


// Open device
pub fn dev_open(path: &str) -> Option<usize>
{
	let flags = OpenFlag::CREATE as usize | OpenFlag::DIRECTORY as usize;
	crate::libcore::sys::sc::open(path, flags)
}


// Disk free
pub fn diskfree() -> usize
{
	disksize() - diskused()
}


// TODO: Shorten the imports
// Disk size
pub fn disksize() -> usize
{
	(crate::libcore::fs::sblk::SBlk::read().blkcount as usize) * crate::libcore::fs::ata::BLKSIZE
}


// TODO: Shorten the imports
// Disk used
pub fn diskused() -> usize
{
	(crate::libcore::fs::sblk::SBlk::read().alloc_count as usize) * crate::libcore::fs::ata::BLKSIZE
}


// Directory name
pub fn dname(pname: &str) -> &str
{
	let n = pname.len();
	let i = match pname.rfind('/')
	{
		Some(0) => 1,
		Some(i) => i,
		None => n,
	};

	&pname[0..i]
}


// Open file
pub fn file_open(path: &str) -> Option<usize>
{
	let flags = 0;
	crate::libcore::sys::sc::open(path, flags)
}


// File name
pub fn fname(pname: &str) -> &str
{
	let n = pname.len();
	let i = match pname.rfind('/')
	{
		Some(i) => i + 1,
		None => 0,
	};

	&pname[i..n]
}


// Info
pub fn info(pname: &str) -> Option<FileInfo>
{
	DirectoryEntry::open(pname).map(|e| e.info())
}


// New file
pub fn new_file(path: &str) -> Option<usize>
{
	let flags = OpenFlag::CREATE as usize;
	crate::libcore::sys::sc::open(path, flags)
}


// Read to bytes
pub fn read_to_bytes(path: &str) -> Result<Vec<u8>, ()>
{
	if let Some(info) = crate::libcore::sys::sc::info(&path)
	{
		let res = if info.isdev()
		{
			dev_open(&path)
		}
		else
		{
			file_open(&path)
		};

		if let Some(handle) = res
		{
			let mut buffer = vec![0; info.size() as usize];
			if let Some(bytes) = crate::libcore::sys::sc::read(handle, &mut buffer)
			{
				buffer.resize(bytes, 0);
				crate::libcore::sys::sc::close(handle);
				return Ok(buffer);
			}
		}
	}
	Err(())
}


// Read to string
pub fn read_to_str(path: &str) -> Result<String, ()>
{
	let buffer = read_to_bytes(path)?;
	Ok(String::from_utf8_lossy(&buffer).to_string())
}


// Real path
pub fn rpath(pname: &str) -> String
{
	if pname.starts_with('/')
	{
		pname.into()
	}
	else
	{
		let dname = crate::libcore::sys::proc::directory();
		let sep = if dname.ends_with('/') { "" } else { "/" };
		format!("{}{}{}", dname, sep, pname)
	}
}


// Write
pub fn write(path: &str, buffer: &[u8]) -> Result<usize, ()>
{
	if let Some(handle) = new_file(&path)
	{
		if let Some(bytes) = crate::libcore::sys::sc::write(handle, buffer)
		{
			crate::libcore::sys::sc::close(handle);
			return Ok(bytes);
		}
	}
	Err(())
}
