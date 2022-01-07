// src/io/stdin.rs
//
// Implements stdin in the kernel.

/*
	IMPORTS
*/

use alloc::{string::{String, ToString}, vec};



// The stdin struct
pub struct Stdin;


// Implementation of the stdin struct
impl Stdin
{
	fn new() -> Self
	{
		Self {}
	}

	// Read character
	pub fn readchar(&self) -> Option<char>
	{
		let mut buffer = vec![0; 4];
		if let Some(bytes) = syscall::read(0, &mut buf)
		{
			if bytes > 0
			{
				buf.resize(bytes, 0);
				return Some(String::from_utf8_lossy(&buffer).to_string().remove(0));
			}
		}
		None
	}


	// Read line
