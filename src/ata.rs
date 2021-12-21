// libraries/fs/ata.rs

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::print;

use alloc::string::String;
use alloc::vec::Vec;
use bit_field::BitField;
use core::fmt;
use core::hint::spin_loop;
use lazy_static::lazy_static;
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
	screg: Port<u8>,
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
			screg: Port::new(iobase + 2),
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
			crate::time::nwait(5);
			self.ctlreg.write(0);
			crate::time::nwait(2000);
		}
	}

	fn wait(&mut self)
	{
		crate::time::nwait(400);
	}

	fn writecmd(&mut self, cmd: Cmd)
	{
		unsafe
		{
			self.cmdreg.write(cmd as u8);
		}
	}

	fn stat(&mut self) -> u8
	{
		unsafe
		{
			self.statreg.read()
		}
	}

	fn lba1(&mut self) -> u8
	{
		unsafe
		{
			self.lba1reg.read()
		}
	}

	fn lba2(&mut self) -> u8
	{
		unsafe
		{
			self.lba2reg.read()
		}
	}

	fn dataread(&mut self) -> u16
	{
		unsafe
		{
			self.datareg.read()
		}
	}

	fn datawrite(&mut self, data: u16)
	{
		unsafe
		{
			self.datareg.write(data)
		}
	}

	fn busyloop(&mut self)
	{
		self.wait();
		let start = crate::clock::uptime();
		while self.busy()
		{
			if crate::clock::uptime() - start > 1.0
			{
				return self.reset();
			}
			spin_loop();
		}
	}

	fn busy(&mut self) -> bool
	{
		self.stat().get_bit(Stat::BUSY as usize)
	}

	fn err(&mut self) -> bool
	{
		self.stat().get_bit(Stat::ERR as usize)
	}

	fn ready(&mut self) -> bool
	{
		self.stat().get_bit(Stat::READY as usize)
	}

	fn driveselect(&mut self, drive: u8)
	{
		let driveid = 0xA0 | (drive << 4);
		unsafe
		{
			self.drivereg.write(driveid);
		}
	}

	fn debug(&mut self)
	{
		self.wait();
		unsafe
		{
			print!("[INFO] DRIVE REGISTER: 0b{:0b}\n", self.drivereg.read());
			print!("[INFO] STATUS: 0b{:08b}\n", self.statreg.read());
		}
	}

	fn setup(&mut self, drive: u8, blk: u32)
	{
		let driveid = 0xE0 | (drive << 4);
		unsafe
		{
			self.drivereg.write(driveid | ((blk.get_bits(24..28) as u8) & 0x0F));
			self.screg.write(1);
			self.lba0reg.write(blk.get_bits(0..8) as u8);
			self.lba1reg.write(blk.get_bits(8..16) as u8);
			self.lba2reg.write(blk.get_bits(16..24) as u8);
		}
	}

	pub fn driveidentify(&mut self, drive: u8) -> Option<[u16; 256]>
	{
		self.reset();
		self.wait();
		self.driveselect(drive);
		self.wait();
		unsafe
		{
			self.screg.write(0);
			self.lba0reg.write(0);
			self.lba1reg.write(0);
			self.lba2reg.write(0);
		}

		self.writecmd(Cmd::Identify);
		self.wait();

		if self.stat() == 0
		{
			return None;
		}

		self.busyloop();

		if self.lba1() != 0 || self.lba2() != 0
		{
			return None;
		}

		for i in 0..
		{
			if i == 25
			{
				self.reset();
				return None;
			}
			if self.err()
			{
				return None;
			}
			if self.ready()
			{
				break;
			}
			self.wait();
		}

		let mut res = [0; 256];
		for i in 0..256
		{
			res[i] = self.dataread();
		}

		Some(res)
	}

	pub fn read(&mut self, drive: u8, blk: u32, buf: &mut [u8])
	{
		assert!(buf.len() == BLKSIZE);
		self.setup(drive, blk);
		self.writecmd(Cmd::Read);
		self.busyloop();
		for i in 0..256
		{
			let data = self.dataread();
			buf[i * 2] = data.get_bits(0..8) as u8;
			buf[i * 2 + 1] = data.get_bits(8..16) as u8;
		}
	}

	pub fn write(&mut self, drive: u8, blk: u32, buf: &[u8])
	{
		assert!(buf.len() == BLKSIZE);
		self.setup(drive, blk);
		self.writecmd(Cmd::Write);
		self.busyloop();
		for i in 0..256
		{
			let mut data = 0 as u16;
			data.set_bits(0..8, buf[i * 2] as u16);
			data.set_bits(8..16, buf[i * 2 + 1] as u16);
			self.datawrite(data);
		}
		self.busyloop();
	}
}
