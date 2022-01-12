// src/libcore/io/stderr.rs
//
// The stderr struct for the LibertyOS kernel.


// The Stderr struct
pub struct Stderr {}


// Implementation of the Stderr struct
impl Stderr
{
	// New
	pub fn new() -> Self
	{
		Self {}
	}


	// Write
	pub fn write(&self, s: &str)
	{
		crate::libcore::sys::sc::write(2, s.as_bytes());
	}
}


// Stderr function
pub fn stderr() -> Stderr
{
	Stderr::new()
}
