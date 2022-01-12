// src/libcore/io/stdout.rs
//
// Handles standard output.


// The Stdout struct
pub struct Stdout;


// Implementation of the Stdout struct
impl Stdout
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


// Stdout function
pub fn stdout() -> Stdout
{
	Stdout::new()
}
