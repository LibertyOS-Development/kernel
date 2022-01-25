// cmos.rs
//
// Establishes kernel support for the time-keeping ability of the computer.

/*
	IMPORTS
*/

use core::hint::spin_loop;
use bit_field::BitField;
use x86_64::instructions::{interrupts, port::Port};


// Reg enumeration
#[repr(u8)]
enum Reg
{
	Second = 0x00,
	Minute = 0x02,
	Hour = 0x04,
	Day = 0x07,
	Month = 0x08,
	Year = 0x09,
	A = 0x0A,
	B = 0x0B,
	C = 0x0C,
}


// Intr enumeration, creates interrupts for time-keeping/alarms
#[repr(u8)]
enum Intr
{
	Periodic = 1 << 6,
	Alarm = 1 << 5,
	Update = 1 << 4,
}


// CMOS struct
pub struct CMOS
{
	// Address
	address: Port<u8>,

	// Data
	data: Port<u8>,
}


// Real-time (RTC) struct
#[derive(Debug, PartialEq)]
pub struct RTC
{
	// Day
	pub day: u8,

	// Hour
	pub hour: u8,

	// Minute
	pub minute: u8,

	// Month
	pub month: u8,

	// Second
	pub second: u8,

	// Year
	pub year: u16,
}


//  Implementation of the CMOS struct
impl CMOS
{
	// New
	pub fn new() -> Self
	{
		CMOS
		{
			address: Port::new(0x70),
			data: Port::new(0x71),
		}
	}


	// Real-time (RTC)
	pub fn rtc(&mut self) -> RTC
	{
		let mut rtc;
		loop
		{
			self.waitfor_update();
			rtc = self.nocheck_rtc();
			self.waitfor_update();
			if rtc == self.nocheck_rtc()
			{
				break;
			}
		}

		let b = self.readreg(Reg::B);
		if b & 0x04 == 0
		{
			rtc.day = (rtc.day & 0x0F) + ((rtc.day / 16) * 10);
			rtc.hour = ((rtc.hour & 0x0F) + (((rtc.hour & 0x70) / 16) * 10)) | (rtc.hour & 0x80);
			rtc.minute = (rtc.minute & 0x0F) + ((rtc.minute / 16) * 10);
			rtc.month = (rtc.month & 0x0F) + ((rtc.month / 16) * 10);
			rtc.second = (rtc.second & 0x0F) + ((rtc.second / 16) * 10);
			rtc.year = (rtc.year & 0x0F) + ((rtc.year / 16) * 10);
		}

		if (b & 0x02 == 0) && (rtc.hour & 0x80 == 0)
		{
			rtc.hour = ((rtc.hour & 0x7F) + 12) % 24;
		}

		rtc.year += 2000;
		rtc
	}


	// Unchecked real-time (RTC)
	fn nocheck_rtc(&mut self) -> RTC
	{
		RTC
		{
			day: self.readreg(Reg::Day),
			hour: self.readreg(Reg::Hour),
			minute: self.readreg(Reg::Minute),
			month: self.readreg(Reg::Month),
			second: self.readreg(Reg::Second),
			year: self.readreg(Reg::Year) as u16,
		}
	}


	// Enable interrupts
	fn enable_intr(&mut self, intr: Intr)
	{
		interrupts::without_interrupts(||
		{
			self.disablenmi();
			unsafe
			{
				self.address.write(Reg::B as u8);
				let prev = self.data.read();
				self.address.write(Reg::B as u8);
				self.data.write(prev | intr as u8);
			}
			self.enablenmi();
			self.notify_intrend();
		});
	}


	// Enable periodic interrupts
	pub fn enable_periodintr(&mut self)
	{
		self.enable_intr(Intr::Periodic);
	}

	
	// Set periodic interrupt rate
	pub fn set_periodintr_rate(&mut self, rate: u8)
	{
		interrupts::without_interrupts(||
		{
			self.disablenmi();
			unsafe
			{
				self.address.write(Reg::A as u8);
				let prev = self.data.read();
				self.address.write(Reg::A as u8);
				self.data.write((prev & 0xF0) | rate);
			}
			self.enablenmi();
			self.notify_intrend();
		});
	}


	// Wait for update to finish
	fn waitfor_update(&mut self)
	{
		while self.updating()
		{
			spin_loop();
		}
	}


	// Check if time is being updated
	fn updating(&mut self) -> bool
	{
		unsafe
		{
			self.address.write(Reg::A as u8);
			self.data.read().get_bit(7)
		}
	}


	// Read registers
	fn readreg(&mut self, reg: Reg) -> u8
	{
		unsafe
		{
			self.address.write(reg as u8);
			self.data.read()
		}
	}


	// Enable alarm interrupt
	pub fn enable_alarmintr(&mut self)
	{
		self.enable_intr(Intr::Alarm);
	}


	// Enable update interrupt
	pub fn enable_updateintr(&mut self)
	{
		self.enable_intr(Intr::Update);
	}


	// Notify when interrupt has ended
	pub fn notify_intrend(&mut self)
	{
		unsafe
		{
			self.address.write(Reg::C as u8);
			self.data.read();
		}
	}


	// Enable NMI
	fn enablenmi(&mut self)
	{
		unsafe
		{
			let prev = self.address.read();
			self.address.write(prev & 0x7F);
		}
	}


	// Disable NMI
	fn disablenmi(&mut self)
	{
		unsafe
		{
			let prev = self.address.read();
			self.address.write(prev | 0x80);
		}
	}
}

