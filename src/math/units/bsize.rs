// src/math/units/bsize.rs
//
// This module provides a simple method for showing the size of files in a better format.

use core::fmt;

#[repr(transparent)]
pub struct ByteSize(usize);

impl ByteSize
{
	pub const fn new(val: usize) -> ByteSize
	{
		ByteSize(val)
	}
}

impl fmt::Display for ByteSize
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		//TODO: Add more, virtually impossible sizes.
		let units = &["B", "KB", "MB", "GB", "TB"];
		let mut val = self.0;
		let mut i = 0;
		let mut unit = units[0];
		while val >= 1024 && i + i < units.len()
		{
			val /= 1024;
			unit = units[i + 1];
			i += 1;
		}
		write!(f, "{}{}", val, unit)
	}
}
