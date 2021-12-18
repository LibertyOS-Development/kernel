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
			self.
