// src/arch/x64/mod.rs
//
// This is the mod.rs file for the x64 (64-bit) arch support for LibertyOS.

pub mod address;
pub mod instructions;
pub mod reg;
pub mod structures;

// An enumeration to represent the different rings of protection.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PrivLvl
{
	// Ring 0: Most amount of privilege, including access to all system resources, such as BIOS, interrupt-handlers, memory-management, etc.
	R0 = 0,

	// Ring 1: High level of privilege, including access to certain system resources and processes, according to restrictions established by the kernel.
	R1 = 1,

	// Ring 2: Moderate level of privilege, including limited access to system resources and processes, according to the restrictions established by the kernel.
	R2 = 2,

	// Ring 3: Least amount of privilege, usually assigned to applications that do not require critical system resources, such as user-applications. Operations within R3 can always request access to resources outside of the privilege level, from other levels.
	R3 = 3,
}


// Implementation of the PrivLvl enumeration:
impl PrivLvl
{
	// Create new PrivLvl, from a specific numeric value.
 	// NOTE: Value must be between 0 and 4.
	pub fn from_u16(value: u16) -> PrivLvl
	{
		match value
		{
			// Ring 0
			0 => PrivLvl::R0,

			// Ring 1
			1 => PrivLvl::R1,

			// Ring 2
			2 => PrivLvl::R2,

			// Ring 3
			3 => PrivLvl::R3,
			// This error is returned if the specified value is outside of the accepted range.
			i => panic!("[ERR] INVALID PRIVILEGE LEVEL: {}", i),
		}
	}
}
