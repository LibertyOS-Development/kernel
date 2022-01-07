// src/libcore/fs/ata.rs
//
// Used to work with ATA devices.

/*
	IMPORTS
*/

use alloc::{string::String, vec::Vec};
use bit_field::BitField;
use core::{convert::TryInto, fmt, hint::spin_loop};
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::port::{Port, PortReadOnly, PortWriteOnly};


pub const BLKSIZE: usize = 512;


#[repr(u16)]
#[derive(Debug, Clone, Copy)]
enum Command
{
	ID = 0xEC,
	Read = 0x20,
	Write = 0x30,
}


enum IDResponse
{
	ATA([u16; 256]),
	ATAPI,
	SATA,
	None,
}


#[repr(usize)]
#[derive(Debug, Clone, Copy)]
enum Status
{
	// Error
	ERR = 0,

	// Index
	IDX = 1,

	// CORR
	CORR = 2,

	// Data request
	DRQ = 3,

	// DSC
	DSC = 4,

	// DF
	DF = 5,

	// Device ready
	DEVREADY = 6,

	// Busy
	BUSY = 7,
}


#[derive(Debug, Clone)]
pub struct Bus
{
	id: u8,
	irq: u8,

	alt_status_reg: PortReadOnly<u8>,
	cmd_reg: PortWriteOnly<u8>,
	ctl_reg: PortWriteOnly<u8>,
	data_reg: Port<u16>,
	drive_reg: Port<u8>,
	drive_blkess_reg: PortReadOnly<u8>,
	err_reg: PortReadOnly<u8>,
	features_reg: PortWriteOnly<u8>,
	sectcount_reg: Port<u8>,
	status_reg: PortReadOnly<u8>,

	lba0_reg: Port<u8>,
	lba1_reg: Port<u8>,
	lba2_reg: Port<u8>,
}

