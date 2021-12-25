// src/noblkio.rs
//
// This module provides the kernel with a non-blocking I/O layer.

use core::fmt;

// Non-blocking result
pub type Result<T, E> = ::core::result::Result<T, Err<E>>;

// Non-blocking error
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Err<E>
{
	 // Other error type
	Other(E),
	// Requires blocking behavior
	WouldBlk,
}

impl<E> fmt::Debug for Err<E>
where
	E: fmt::Debug,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		match *self
		{
			Err::Other(ref e) => fmt::Debug::fmt(e, f),
			Err::WouldBlk => f.write_str("[INFO] WOULDBLK"),
		}
	}
}


impl<E> Err<E>
{
	// This maps an Err<E> to an Err<T>.
	pub fn map<T, F>(self, op: F) -> Err<T>
	where
		F: FnOnce(E) -> T,
	{
		match self
		{
			Err::Other(e) => Err::Other(op(e)),
			Err::WouldBlk => Err::WouldBlk,
		}
	}
}
