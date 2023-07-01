// src/libcore/sys/rsdp.rs
//
// Support for RSDP (Root System Descriptor Table).

#![allow(non_camel_case_types)]

/*
	IMPORTS
*/

use core::{mem, ops::Range, slice, str};
use log::warn;


// RDSP Error enumeration
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RSDP_ERR
{
	// Invalid RSDP
	INVALID_RSDP,

	// Bad signature
	BAD_SIG,

	// Invalid OEMID
	INVALID_OEMID,

	// Invalid checksum
	INVALID_CS,
}


// The RSDP struct
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct RSDP
{
	// Signature
	sig: [u8; 8],

	// Checksum
	cs: u8,

	// OEMID
	oemid: [u8; 6],

	// Revision
	rev: u8,

	// RSDT address
	address: u32,


	// For ACPI v2.0+

	// Length
	len: u32,

	// XSDT address
	xsdt: u64,

	// Extended checksum
	extcs: u8,

	// Reserved
	reserved: [u8; 3],
}


// Implementation of the RSDP struct

