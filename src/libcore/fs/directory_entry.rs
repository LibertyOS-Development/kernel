// src/libcore/fs/directory_entry.rs
//
// Directory entry/operations functionality.

/*
	IMPORTS
*/

use alloc::string::String;
use crate::libcore::fs::{dname, fname, rpath, FileType, directory::Directory};


// Basic DirectoryEntry struct
#[derive(Clone)]
pub struct DirectoryEntry
{
	address: u32,
	directory: Directory,
	name: String,
	size: u32,
	time: u64,
	tp: FileType,
}


// FileInfo struct
#[derive(Debug)]
pub struct FileInfo
{
	size: u32,
	time: u64,
	tp: FileType,
}


// Implementation of the DirectoryEntry struct
impl DirectoryEntry
{
	// Address
	pub fn address(&self) -> u32
	{
		self.address
	}

	// Directory
	pub fn directory(&self) -> Directory
	{
		self.directory
	}

	// File is empty
	pub fn empty(&self) -> bool
	{
		Self::len_null() == self.len()
	}

	// Info
	pub fn info(&self) -> FileInfo
	{
		FileInfo
		{
			tp: self.tp,
			size: self.size,
			time: self.time
		}
	}

	// Is a device
	pub fn isdev(&self) -> bool
	{
		self.tp == FileType::Dev
	}

	// Is a directory
	pub fn isdir(&self) -> bool
	{
		self.tp == FileType::Directory
	}

	// Is a file
	pub fn isfile(&self) -> bool
	{
		self.tp == FileType::File
	}

	// Length
	pub fn len(&self) -> usize
	{
		Self::len_null() + self.name.len()
	}

	// Empty length
	pub fn len_null() -> usize
	{
		1 + 4 + 4 + 8 + 1
	}

	// Name
	pub fn name(&self) -> String
	{
		self.name.clone()
	}

	// New
	pub fn new(directory: Directory, tp: FileType, address: u32, size: u32, time: u64, name: &str) -> Self
	{
		let name = String::from(name);
		Self
		{
			directory,
			tp,
			address,
			size,
			time,
			name
		}
	}

	// Open
	pub fn open(pname: &str) -> Option<Self>
	{
		let pname = rpath(pname);
		let dname = dname(&pname);
		let fname = fname(&pname);

		if let Some(directory) = Directory::open(dname)
		{
			return directory.find(fname);
		}

		None
	}

	// Size
	pub fn size(&self) -> u32
	{
		self.size
	}


	// Time
	pub fn time(&self) -> u64
	{
		self.time
	}


	// File type
	pub fn tp(&self) -> FileType
	{
		self.tp
	}
}


// Implementation of the FileInfo struct
impl FileInfo
{
	// Is a device
	pub fn isdev(&self) -> bool
	{
		self.tp == FileType::Dev
	}

	// Is a directory
	pub fn isdir(&self) -> bool
	{
		self.tp == FileType::Directory
	}

	// Is a file
	pub fn isfile(&self) -> bool
	{
		self.tp == FileType::File
	}

	// New
	pub fn new() -> Self
	{
		Self
		{
			size: 0,
			time: 0,
			tp: FileType::File,
		}
	}

	// Size
	pub fn size(&self) -> u32
	{
		self.size
	}

	// Time
	pub fn time(&self) -> u64
	{
		self.time
	}
}
