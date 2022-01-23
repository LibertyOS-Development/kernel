// src/libcore/fs/dev.rs
//
// Basic device functionality for working with filesystems.

/*
	IMPORTS
*/

use crate::{libcore::{fs::directory::Directory, fs::blk::LinkBlk, fs::file::File, fs::FileIO, fs::fname, fs::rpath, sys::{console::Console, rand::Random}}};

// Device enumeration
#[derive(Debug, Clone)]
pub enum Device
{
	File(File),
	Console(Console),
	Random(Random),
}

// Basic device type enumeration
#[repr(u8)]
pub enum DevType
{
	File = 0,
	Console = 1,
	Random = 2,
}


// Implementation of the Device enumeration
impl Device
{
	// Create
	pub fn create(pname: &str) -> Option<Self>
	{
		let pname = rpath(pname);
		let dname = crate::libcore::fs::dname(&pname);
		let fname = fname(&pname);
		if let Some(directory) = Directory::open(dname)
		{
			if let Some(directory_entry) = directory.new_dev(fname)
			{
				return Some(Device::File(directory_entry.into()))
			}
		}
		None
	}


	// New
	fn new(i: u8) -> Self
	{
		match i
		{
			i if i == DevType::Console as u8 => Device::Console(Console::new()),
			i if i == DevType::Random as u8 => Device::Random(Random::new()),
			_ => unimplemented!(),
		}
	}


	// Open
	pub fn open(pname: &str) -> Option<Self>
	{
		let pname = rpath(pname);
		let dname = crate::libcore::fs::dname(&pname);
		let fname = fname(&pname);
		if let Some(directory) = Directory::open(dname)
		{
			if let Some(directory_entry) = directory.find(fname)
			{
				if directory_entry.isdev()
				{
					let blk = LinkBlk::read(directory_entry.address());
					let data = blk.data();
					return Some(Self::new(data[0]));
				}
			}
		}
		None
	}
}


// Implementation of the FileIO trait for the Device enumeration
impl FileIO for Device
{
	// Read
	fn read(&mut self, buffer: &mut [u8]) -> Result<usize, ()>
	{
		match self
		{
			Device::File(io) => io.read(buffer),
			Device::Console(io) => io.read(buffer),
			Device::Random(io) => io.read(buffer),
		}
	}

	// Write
	fn write(&mut self, buffer: &[u8]) -> Result<usize, ()>
	{
		match self
		{
			Device::File(io) => io.write(buffer),
			Device::Console(io) => io.write(buffer),
			Device::Random(io) => io.write(buffer),
		}
	}
}
