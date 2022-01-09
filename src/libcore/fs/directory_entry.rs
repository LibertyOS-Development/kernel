// src/libcore/fs/directory_entry.rs
//
// Directory entry/operations functionality.

/*
	IMPORTS
*/

use alloc::string::String;
use crate::libcore::fs::{FileType, directory::Directory};


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

	// Size
	pub fn size(&self) -> u32
	{
		self.size
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
	pub fn dev(&self) -> bool
	{
		self.tp == FileType::Dev
	}

	// Is a directory
	pub fn dir(&self) -> bool
	{
		self.tp == FileType::Directory
	}

	// Is a file
	pub fn file(&self) -> bool
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
