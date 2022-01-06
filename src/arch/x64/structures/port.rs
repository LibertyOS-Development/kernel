// src/arch/x64/structures/port.rs
//
// Basic I/O port functionality.

// Read
pub trait ReadPort
{
	// Read whatever value is contained in "Self", from a specific port.
	unsafe fn port_read(port: u16) -> Self;
}

// Write
pub trait WritePort
{
	// Write whatever value is contained in "Self", to a specific port.
	unsafe fn port_write(port: u16, value: Self);
}
