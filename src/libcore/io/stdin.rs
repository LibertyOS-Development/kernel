// src/io/stdin.rs
//
// Implements stdin in the kernel.

/*
	IMPORTS
*/

use alloc::{string::{String, ToString}, vec};

use crate::libcore::sys::sc::read;


// The stdin struct
pub struct Stdin;


// Implementation of the stdin struct
impl Stdin
{
	// New
	pub fn new() -> Self
	{
		Self {}
	}


	// Read character
	pub fn readchar(&self) -> Option<char>
	{
		let mut buffer = vec![0; 4];
		if let Some(bytes) = crate::libcore::sys::sc::read(0, &mut buffer)
		{
			if bytes > 0
			{
				buffer.resize(bytes, 0);
				return Some(String::from_utf8_lossy(&buffer).to_string().remove(0));
			}
		}
		None
	}


	// Read line
	pub fn readln(&self) -> String
	{
		let mut buffer = vec![0; 256];
		if let Some(bytes) = crate::libcore::sys::sc::read(0, &mut buffer)
		{
			buffer.resize(bytes, 0);
			String::from_utf8_lossy(&buffer).to_string()
		}
		else
		{
			String::new()
		}
	}
}


// Stdin function
pub fn stdin() -> Stdin
{
	Stdin::new()
}
