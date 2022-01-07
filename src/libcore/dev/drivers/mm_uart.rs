// src/dev/drivers/mm_uart.rs
//
// This module adds support for memory-mapped addresses (for use by UART interfaces).

use core::{fmt, sync::atomic::{AtomicPtr, Ordering}};
use crate::dev::drivers::uart::LineStatFlag;
use crate::waitfor;

// Layout of a memory-mapped UART:
pub struct MMIOSerial
{
	data: AtomicPtr<u8>,
	inten: AtomicPtr<u8>,
	fifoctl: AtomicPtr<u8>,
	lnctl: AtomicPtr<u8>,
	modemctl: AtomicPtr<u8>,
	lnstat: AtomicPtr<u8>,
}

impl MMIOSerial
{
	pub const unsafe fn new(base: usize) -> Self
	{
		let baseptr = base as *mut u8;
		Self
		{
			data: AtomicPtr::new(baseptr),
			inten: AtomicPtr::new(baseptr.add(1)),
			fifoctl: AtomicPtr::new(baseptr.add(2)),
			lnctl: AtomicPtr::new(baseptr.add(3)),
			modemctl::AtomicPtr::new(baseptr.add(4)),
			lnstat: AtomicPtr::new(baseptr.add(5)),
		}
	}

	// Initialize memory-mapped UART interface
	pub fn init(&mut self)
	{
		let self_inten = self.inten.load(Ordering::Relaxed);
		let self_lnctl = self.lnctl.load(Ordering::Relaxed);
		let self_data = self.data.load(Ordering::Relaxed);
		let self_fifoctl = self.fifoctl.load(Ordering::Relaxed);
		let self_modemctl = self.modemctl.load(Ordering::Relaxed);
		unsafe
		{
			// Disable interrupts:
			self_inten.write(0x00);
			// Enable DLAB:
			self_lnctl.write(0x80);
			// Configure DLL/DLM to set maximum speed to 38400 b/s:
			self_data.write(0x03);
			self_inten.write(0x00);
			// Disable DLAB, set data word-length to 8 bits:
			self_lnctl.write(0x03);
			// Enable FIFO, clear TX/RX queues, set interrupt watermark to 14 bytes:
			self_fifoctl.write(0xC7);
			// Declare data as being ready for the terminal, request permission to send, enable aux. output number 2 as CPU interrupt line:
			self_modemctl.write(0x0B);
			// Enable interrupts:
			self_inten.write(0x01);
		}
	}

	fn lnstat(&mut self) -> LineStatFlag
	{
		unsafe
		{
			LineStatFlag::from_bits_truncate(*self.lnstat.load(Ordering::Relaxed))
		}
	}

	// Send a byte via the serial port:
	pub fn send(&mut self, data: u8)
	{
		let self_data = seld.data.load(Ordering::Relaxed);
		unsafe
		{
			match data
			{
				8 | 0x7F =>
				{
					waitfor!(self.lnstat().contains(LineStatFlag::OUTPEMPTY));
					self_data.write(8);

					waitfor!(self.lnstat().contains(LineStatFlag::OUTPEMPTY));
					self_data.write(b' ');

					waitfor!(self.lnstat().contains(LineStatFlag::OUTPEMPTY));
					self_data.write(8)
				}

				_ =>
				{
					waitfor!(self.lnstat().contains(LineStatFlags::OUTPEMPTY));
					self_data.write(data);
				}
			}
		}
	}

	// Receive a byte via the serial port:
	pub fn receive(&mut self) -> u8
	{
		let self_data = self.data.load(Ordering::Relaxed);
		unsafe
		{
			waitfor!(self.lnstat().contains(LineStatFlag::INPFULL));
			self_data.read()
		}
	}
}


impl fmt::Write for MMIOSerial
{
	fn write_str(&mut self, s: &str) -> fmt::Result
	{
		for byte in s.bytes()
		{
			self.send(byte);
		}
		Ok(())
	}
}
