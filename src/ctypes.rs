// src/ctypes.rs
//
// Provides kernel with support for various C types.

#![allow(non_camel_case_types)]
//TODO: USE BFLAG
use bitflags::bitflags;

// 16-bit integer
pub type cint_16 = i16;

// 32-bit integer
pub type cint_32 = i32;

// 64-bit integer
pub type cint_64 = i64;

// Unsigned 32-bit integer
pub type cuint_32 = u32;

// Unsigned 64-bit integer
pub type cuint_64 = u64;


// "cint" equals a 32-bit integer.
pub type cint = cint_32;

// "cuint" equals an unsigned 32-bit integer.
pub type cuint = cuint_32;

// "cshort" equals a 64-bit integer.
pub type cshort = cint_64;

// "clong" equals a 64-bit integer.
pub type clong = cint_64;

// "culong" equals an unsigned 64-bit integer.
pub type culong = cuint_64;

// "ctime" equals a 64-bit integer.
pub type ctime = cint_64;

// "csusec" equals a 64-bit integer.
pub type csusec = cint_64;

// "cclockid" equals "cint", which equals a 32-bit integer.
pub type cclockid = cint;

// "cnfds" equals "culong", which equals an unsigned 64-bit integer.
pub type cnfds = culong;

// "csize" equals "culong", which equals an unsigned 64-bit integer.
pub type csize = culong;

// "coff" equals an unsigned 64-bit integer.
pub type coff = cuint_64;

pub const CLOCK_REALTIME: cclockid = 0;
pub const CLOCK_MONO: cclockid = 1;

// TODO: USE BFLAG
bitflags!
{
	pub struct MMapProt: cint
	{
		const PROTREAD = 1;
		const PROTWRITE = 2;
		const PROTEXEC = 4;
	}
}

//TODO: USE BFLAG
bitflags!
{
	pub struct MMapFlags: cint
	{
		const MAPPRIV = 0x02;
		const MAPFIXED = 0x10;
		const MAPANON = 0x20;
	}
}
