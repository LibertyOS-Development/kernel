// src/fs/directory.rs
//
// Basic functionality of directories.

use alloc::string::String;
use core::convert::From;

#[derive(Debug, Clone, Copy)]
pub struct Directory
{
	address: u32,
}

//impl From<DirectoryEntry> for Directory
