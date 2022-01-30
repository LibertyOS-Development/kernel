// src/libcore/sys/boot/multiboot/memmap.rs
//
// Memory-mapping support for multiboot.

// EFI memory-map descriptor
#[derive(Debug)]
#[repr(C)]
pub struct EFIMemDesc
{
	// Attributes
	attr: u64,

	// Padding
	_pad: u32,

	// Physical address
	padr: u64,

	// Number of pages
	npg: u64,

	// Type
	tp: u32,

	// Virtual address
	vadr: u64,
}


// Implementation of the EFIMemDesc struct
impl EFIMemDesc
{
	// Physical address  
	pub fn padr(&self) -> u64
	{
		self.padr
	}


	// Size
	pub fn size(&self) -> u64
	{
		self.npg * 4096
	}


	// Virtual address
	pub fn vadr(&self) -> u64
	{
		self.vadr
	}
}
