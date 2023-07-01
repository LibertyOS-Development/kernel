// src/libcore/sys/boot/multiboot/efi.rs
//
// MBI tags for working with UEFI.

// EFI system table (32-bit)
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct EFI32
{
	// Pointer
	ptr: u32,

	// Type
	tp: TagTp,

	// Size
	size: u32,
}
