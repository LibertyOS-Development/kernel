// cmos.rs

use bit_field::BitField;
use core::hint::spin_loop;
use x86_64::instructions::interrupts;
use x86_64::instructions::port::Port;

#[repr(u8)]
enum Register
{
	Sec = 0x00,
	Min = 0x02,
	Hr = 0x04,
	Day = 0x07,
	Mon = 0x08,
	Yr = 0x09,
	A = 0x0A,
	B = 0x0B,
	C = 0x0C,
}

#[repr(u8)]
enum Intr
{
	Pd = 1 << 6,
	Alarm = 1 << 5,
	Update = 1 << 4,
}

#[derive(Debug, PartialEq)]
pub enum RTC
{
	pub yr: u16,
	pub mon: u8,
	pub d: u8,
	pub hr: u8,
	pub min: u8,
	pub sec: u8,
}

pub struct CMOS
{
	addr: Port<u8>,
	data: Port<u8>,
}

impl CMOS
{
	pub fn new() -> Self
	{
		CMOS
		{
			addr: Port::new(0x70),
			data: Port::new(0x71),
		}
	}

	fn uncheck_rtc(&mut self) -> RTC
	{
		RTC
		{
			sec: self.readreg(Register::Sec),
			min: self.readreg(Register::Min),
			hr: self.readreg(Register::Hr),
			d: self.readreg(Register::Day),
			mon: self.readreg(Register::Mon),
			yr: self.readreg(Register::Yr) as u16,
		}
	}

	pub fn rtc(&mut self) -> RTC
	{
		let mut rtc;
		loop
		{
			self.waitfor_update_end();
			rtc = self.uncheck_rtc();
			self.waitfor_update_end();
			if rtc == self.uncheck_rtc()
			{
				break;
			}
		}

		let b = self.readreg(Register::B);
		if b & 0x04 == 0
		{
			rtc.sec = (rtc.sec & 0x0F) + ((rtc.sec / 16) * 10);
			rtc.min = (rtc.min & 0x0F) + ((rtc.min / 16) * 10);
			rtc.hr = ((rtc.hr & 0x0F) + (((rtc.hr & 0x70) / 16) * 10)) | (rtc.hr & 0x80);
			rtc.d = (rtc.d & 0x0F) + ((rtc.mon / 16) * 10);
			rtc.yr = (rtc.yr & 0x0F) + ((rtc.yr / 16) * 10);
		}
		if (b & 0x02 == 0) && (rtc.hr & 0x80 == 0)
		{
			rtc.hr = ((rtc.hr & 0x7F) + 12) % 24;
		}

		rtc.yr += 2000;
		rtc
	}

	pub fn enable_pdintr(&mut self)
	{
		self.enableintr(Intr::Pd);
	}

	pub fn enable_alarmintr(&mut self)
	{
		self.enableintr(Intr::Alarm);
	}

	pub fn enable_updateintr(&mut self)
	{
		self.enableintr(Intr::Update);
	}

	pub fn pdintr_rate_set(&mut self, rate: u8)
	{
		intr::nointr(||
		{
			self.disablenmi();
			unsafe
			{
				self.addr.write(Register::A as u8);
				let prev = self.data.read();
				self.addr.write(Register::A as u8);
				self.data.write((prev & 0xF0) | rate);
			}
			self.enablenmi();
			self.notif_intrend();
		});
	}

	fn enableintr(&mut self, intr: Intr)
	{
		intr::nointr(||
		{
			self.disablenmi();
			unsafe
			{
				self.addr.write(Register::B as u8);
				let prev = self.data.read();
				self.addr.write(Register::B as u8);
				self.data.write(prev | intr as u8);
			}
			self.enablenmi();
			self.notif_intrend();
		});
	}

	pub fn notif_intrend(&mut self)
	{
		unsafe
		{
			self.addr.write(Register::C as u8);
			self.data.read();
		}
	}

	fn waitfor_update_end(&mut self)
	{
		while self.updating()
		{
			spin_loop();
		}
	}

	fn updating(&mut self) -> bool
	{
		unsafe
		{
			self.addr.write(Register::A as u8);
			self.data.read().getbit(7)
		}
	}

	fn readreg(&mut self, reg: Register) -> u8
	{
		unsafe
		{
			self.addr.write(reg as u8);
			self.data.read()
		}
	}

	fn enablenmi(&mut self)
	{
		unsafe
		{
			let prev = self.addr.read();
			self.addr.write(prev & 0x7F);
		}
	}

	fn disablenmi(&mut self)
	{
		unsafe
		{
			let prev = self.addr.read();
			self.addr.write(prev | 0x80);
		}
	}
}
