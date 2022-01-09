// src/libcore/fs/mod.rs
//
// This is the mod.rs file for the libcore::fs module.


pub mod ata;
pub mod blk;
pub mod blkdev;
pub mod bmapblk;
pub mod dev;
pub mod directory;
pub mod directory_entry;
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

/*
// Resource enumeration
pub enum Resource
{
	Device(crate::libcore::fs::dev::Device),
	Directory(crate::libcore::fs::directory::Directory),
	File(crate::libcore::fs::file::File),
}
*/

// Implementation of the OpenFlag enumeration
impl OpenFlag
{
	fn set(&self, flags: usize) -> bool
	{
		flags & (*self as usize) != 0
	}
}


// Directory name
pub fn directoryname(pathname: &str) -> &str
{
	let n = pathname.len();
	let i = match pathname.rfind('/')
	{
		Some(0) => 1,
		Some(i) => i,
		None => n,
	};

	&pathname[0..i]
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
