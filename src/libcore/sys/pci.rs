// src/libcore/sys/pci.rs
//
// Basic PCI functionality for the LibertyOS kernel.


/*
	IMPORTS
*/

use alloc::vec::Vec;
use bit_field::BitField;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::port::Port;


// DevConfig struct
#[derive(Debug, Clone, Copy)]
pub struct DevConfig
{
	// Base addresses
	pub base_addresses: [u32; 6],

	// Bus
	pub bus: u8,

	// Class
	pub class: u8,

	// Command
	pub cmd: u16,

	// Device
	pub dev: u8,

	// Device ID
	pub did: u16,

	// Interrupt Line
	pub intr_ln: u8,

	// Interrupt Pin
	pub intr_pin: u8,

	// Prog
	pub prog: u8,

	// Rev
	pub rev: u8,

	// Status
	pub status: u16,

	// Subclass
	pub subclass: u8,

	// Vendor ID
	pub vid: u16,
}


