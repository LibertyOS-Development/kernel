// libraries/fs/ata.rs

#![allow(dead_code)]
#![allow(unused_imports)]

use alloc::string::String;
use alloc::vec::Vec;
use bit_field::BitField;
use core::fmt;
use core::hint::spin_loop;
use lazy_static::lazy_static;
use libertyos_kernel;
use spin::Mutex;
use x86_64::instructions::port::{Port, PortReadOnly, PortWriteOnly};

pub const BLKSIZE: usize = 512;

#[repr(u16)]
enum Cmd
{
	Read = 0x20,
	Write = 0x30,
	Identify = 0xEC,
}

#[repr(usize)]
enum Stat
{
	ERR = 0,
	IDX = 1,
	CORR = 2,
	DRQ = 3,
	SRV = 4,
	DF = 5,
	READY = 6,
	BUSY = 7,
}

#[derive(Debug, Clone)]
pub struct Bus
{
	id: u8,
	irq: u8,
	datareg: Port<u16>,
	err_reg: PortReadOnly<u8>,
	featreg: PortWriteOnly<u8>,
	sectcount_reg: Port<u8>,
	lba0reg: Port<u8>,
	lba1reg: Port<u8>,
	lba2reg: Port<u8>,
	drivereg: Port<u8>,
	statreg: PortReadOnly<u8>,
	cmdreg: PortWriteOnly<u8>,
	alt_statreg: PortReadOnly<u8>,
	ctlreg: PortWriteOnly<u8>,
	driveblkess_reg: PortReadOnly<u8>,
}

impl Bus
{
	pub fn new(id: u8, iobase: u16, ctlbase: u16, irq: u8) -> Self
	{
		Self
		{
			id, irq,
			datareg: Port::new(iobase + 0),
			err_reg: PortReadOnly::new(iobase + 1),
			featreg: PortWriteOnly::new(iobase + 1),
			sectcount_reg: Port::new(iobase + 2),
			lba0reg: Port::new(iobase + 3),
			lba1reg: Port::new(iobase + 4),
			lba2reg: Port::new(iobase + 5),
			drivereg: Port::new(iobase + 6),
			statreg: PortReadOnly::new(iobase + 7),
			cmdreg: PortWriteOnly::new(iobase + 7),
			alt_statreg: PortReadOnly::new(ctlbase + 0),
			ctlreg: PortWriteOnly::new(ctlbase + 0),
			driveblkess_reg: PortReadOnly::new(ctlbase + 1),
		}
	}

	fn reset(&mut self)
	{
		unsafe
		{
			self.ctlreg.write(4);
			
