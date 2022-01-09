// src/libcore/fs/file.rs
//
// Basic file functionality for working with filesystems.

/*
	IMPORTS
*/

use alloc::{string::{String, ToString}, vec};
use core::convert::From;
use crate::libcore::fs::{directory::Directory, directory_entry::DirectoryEntry};

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

/*
// Implementation of the File struct
impl File
{
	// Create
	pub fn new(path: &str) -> Option<Self>
	{
		
*/
