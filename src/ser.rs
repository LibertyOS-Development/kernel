// src/ser.rs
//
// This module handles basic input and output to/from serial ports.


/*
	IMPORTS
*/

use core::{fmt, fmt::Write};
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;
use x86_64::instructions::interrupts;



lazy_static!
{
	pub static ref SER: Mutex<Serial> = Mutex::new(Serial::new(0x3F8));
}


// Serial struct
pub struct Serial
{
	// Port
	pub port: SerialPort,
}


// Implementation of the Serial struct
impl Serial
{
	// New
	fn new(address: u16) -> Self
	{
		let mut port = unsafe
		{
			SerialPort::new(address)
		};

		port.init();

		Self
		{
			port
		}
	}


	// Write byte
	pub fn wrbyte(&mut self, byte: u8)
	{
		self.port.send(byte);
	}
}


// Implementation of the fmt::Write trait for the Serial struct
impl fmt::Write for Serial
{
	// Write string
	fn write_str(&mut self, s: &str) -> fmt::Result
	{
		for byte in s.bytes()
		{
			self.wrbyte(byte)
		}

		Ok(())
	}
}


#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments)
{
	use core::fmt::Write;
	use x86_64::instructions::interrupts;

	interrupts::without_interrupts(||
	{
		SER.lock().write_fmt(args).expect("[ERR] FAILED TO PRINT TO SERIAL");
	});
}


// Initialize
pub fn init()
{
	crate::libcore::sys::idt::set_irh(4, intrh);
}


// Interrupt handler
fn intrh()
{
	let b = SER.lock().port.receive();
	let c = match b as char
	{
		'\r' => '\n',
		'\x7F' => '\x08',
		c => c,
	};

	crate::libcore::sys::console::keyhandler(c);
}


// Print formatting
pub fn printfmt(args: fmt::Arguments)
{
	interrupts::without_interrupts(||
	{
		SER.lock().write_fmt(args).expect("[ERR] COULD NOT PRINT TO SERIAL");
	})
}


#[macro_export]
macro_rules! serprint {
	($($arg:tt)*) => {
		$crate::ser::_print(format_args!($($arg)*));
	};
}

#[macro_export]
macro_rules! serprintln
{
	() => ($crate::serprint!("\n"));
	($fmt:expr) => ($crate::serprint!(concat!($fmt, "\n")));
	($fmt:expr, $($arg:tt)*) => ($crate::serprint!(concat!($fmt, "\n"), $($arg)*));
}
