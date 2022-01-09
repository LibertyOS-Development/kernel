// src/libcore/fs/dev.rs
//
// Basic device functionality for working with filesystems.


// Basic device type enumeration
#[repr(u8)]
pub enum DevType
{
	File = 0,
	Console = 1,
	Random = 2,
}
