// fs/mod.rs
//
// This is the mod.rs file for the fs module.

/*
	IMPORTS
*/


/*
	MODULES
*/




pub const VERS: u8 = 1;

#[repr(u8)]
pub enum OpenFlag
{
	READ = 1,
	WRITE = 2,
	NEW = 4,
	DIRECTORY = 8,
	DEVICE = 16,
}

impl OpenFlag
{
	fn set(self, flags: usize) -> bool
	{
		flags & (self as usize) != 0
	}
}
