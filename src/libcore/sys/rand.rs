// src/libcore/sys/rand.rs
//
// Random number generation. This module may need to be expanded to support older CPUs.

/*
	IMPORTS
*/

use rand_core::{RngCore, SeedableRng};
use x86_64::instructions::random::RdRand;

use crate::libcore::fs::FileIO;



// Random struct
#[derive(Debug, Clone)]
pub struct Random;


// Implementation of the FileIO trait for the Random struct
impl FileIO for Random
{
	// Read
	fn read(&mut self, buffer: &mut [u8]) -> Result<usize, ()>
	{
		let n = buffer.len();
		for i in 0..n
		{
			buffer[i] = ret_u64() as u8;
		}
		Ok(n)
	}

	// Write
	fn write(&mut self, _buffer: &[u8]) -> Result<usize, ()>
	{
		unimplemented!();
	}
}


// Implementation of the Random struct
impl Random
{
	// New
	pub fn new() -> Self
	{
		Self {}
	}
}


// Returns a random integer (using ret_u64) as an unsigned, 16-bit integer
pub fn ret_u16() -> u16
{
	ret_u64 as u16
}


// Returns a random integer (using ret_u64) as an unsigned, 32-bit integer
pub fn ret_u32() -> u32
{
	ret_u64() as u32
}


// Returns a random, unsigned, 64-bit integer
pub fn ret_u64() -> u64
{
	if let Some(rdrand) = RdRand::new()
	{
		if let Some(rand) = rdrand.get_u64()
		{
			return rand;
		}
	}
	0
}
