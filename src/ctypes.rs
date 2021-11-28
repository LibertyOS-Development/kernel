#![allow(non_camel_case_types)]
//TODO: USE BFLAG
use bitflags::bitflags;

pub type cint_16 = i16;
pub type cint_32 = i32;
pub type cint_64 = i64;
pub type cuint_32 = u32;
pub type cuint_64 = u64;

pub type cint = cint_32;
pub type cuint = cuint_32;
pub type cshort = cint_64;
pub type clong = cint_64;
pub type culong = cuint_64;

pub type ctime = cint_64;
pub type csusec = cint_64;
pub type cclockid = cint;
pub type cnfds = culong;
pub type csize = culong;
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
