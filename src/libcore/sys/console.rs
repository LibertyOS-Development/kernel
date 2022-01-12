// src/libcore/sys/console.rs
//
// Basic console functionality and definitions.


/*
	IMPORTS
*/

use alloc::string::{String, ToString};
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts;

use crate::{print, libcore::fs::FileIO};


/*
	CONSTANTS
*/

// End of text
const END_TEXT: char = '\x03';

// End of transmission
const END_TRANS: char = '\x04';

// Backspace
const BACKSPACE: char = '\x08';

// Escape
const ESC: char = '\x1B';

lazy_static!
{
	pub static ref STDIN: Mutex<String> = Mutex::new(String::new());
	pub static ref ECHO: Mutex<bool> = Mutex::new(true);
	pub static ref RAW: Mutex<bool> = Mutex::new(false);
}


// Console struct
#[derive(Debug, Clone)]
pub struct Console;


// Style struct
#[derive(Clone, Copy)]
pub struct Style
{
	// Foreground
	fg: Option<usize>,

	// Background
	bg: Option<usize>,
}

// Implementation of the Console struct
impl Console
{
	// New
	pub fn new() -> Self
	{
		Self {}
	}
}


// Implementation of the FileIO trait for the Console struct
impl FileIO for Console
{
	// Read
	fn read(&mut self, buffer: &mut [u8]) -> Result<usize, ()>
	{
		let mut s = if buffer.len() == 4
		{
			readchar().to_string()
		}
		else
		{
			readln()
		};

		s.truncate(buffer.len());
		let n = s.len();
		buffer[0..n].copy_from_slice(s.as_bytes());

		Ok(n)
	}

	// Write
	fn write(&mut self, buffer: &[u8]) -> Result<usize, ()>
	{
		let s = String::from_utf8_lossy(buffer);
		let n = s.len();
		// TODO: Add formatting for printing

		Ok(n)
	}
}


// Implementation of the Style struct
impl Style
{
	// Reset
	pub fn reset() -> Self
	{
		Self
		{
			fg: None,
			bg: None
		}
	}
}


// Can print
pub fn canprint(c: char) -> bool
{
	((c as u32) < 0xFF) && crate::libcore::graphics::vga::canprint(c as u8)
}


// Drain
pub fn drain()
{
	interrupts::without_interrupts(||
	{
		STDIN.lock().clear();
	})
}


// Disable echo
pub fn echo_off()
{
	let mut echo = ECHO.lock();
	*echo = false;
}


// Enable echo
pub fn echo_on()
{
	let mut echo = ECHO.lock();
	*echo = true;
}


// Checks whether or not echo has been enabled
pub fn echo_stat() -> bool
{
	*ECHO.lock()
}


// End-of-text
pub fn end_text() -> bool
{
	interrupts::without_interrupts(||
	{
		STDIN.lock().contains(END_TEXT)
	})
}


// End-of-transmission
pub fn end_trans() -> bool
{
	interrupts::without_interrupts(||
	{
		STDIN.lock().contains(END_TRANS)
	})
}


// Key-handler
pub fn keyhandler(key: char)
{
	let mut stdin = STDIN.lock();

	if key == BACKSPACE && !raw_stat()
	{
		if let Some(c) = stdin.pop()
		{
			if echo_stat()
			{
				let n = match c
				{
					END_TEXT | END_TRANS | ESC => 2,
					_ => if (c as u32) < 0xFF
					{
						1
					}
					else
					{
						c.len_utf8()
					},
				};
				printfmt(format_args!("{}", BACKSPACE.to_string().repeat(n)));
			}
		}
	}
	else
	{
		let key = if (key as u32) < 0xFF
		{
			(key as u8) as char
		}
		else
		{
			key
		};

		if echo_stat()
		{
			match key
			{
				END_TEXT => printfmt(format_args!("^C")),
				END_TRANS => printfmt(format_args!("^D")),
				ESC => printfmt(format_args!("^[")),
				_ => printfmt(format_args!("{}", key)),
			};
		}
	}
}

// Print formatting
pub fn printfmt(args: fmt::Arguments)
{
	crate::libcore::graphics::vga::printfmt(args);
}


// Disable raw
pub fn raw_off()
{
	let mut raw = RAW.lock();
	*raw = false;
}


// Enable raw
pub fn raw_on()
{
	let mut raw = RAW.lock();
	*raw = true;
}


// Checks whether or not raw has been enabled
pub fn raw_stat() -> bool
{
	*RAW.lock()
}


// Read character
pub fn readchar() -> char
{
	// Disable echo
	echo_off();

	// Enable raw
	raw_on();

	loop
	{
		crate::time::halt();
		let res = interrupts::without_interrupts(||
		{
			let mut stdin = STDIN.lock();
			if !stdin.is_empty()
			{
				Some(stdin.remove(0))
			}
			else
			{
				None
			}
		});

		if let Some(c) = res
		{
			// Enable echo
			crate::libcore::sys::console::echo_on();

			// Disable raw
			crate::libcore::sys::console::raw_off();
			return c;
		}
	}
}


// Read line
pub fn readln() -> String
{
	loop
	{
		crate::time::halt();
		let res = interrupts::without_interrupts(||
		{
			let mut stdin = STDIN.lock();
			match stdin.chars().next_back()
			{
				Some('\n') =>
				{
					let ln = stdin.clone();
					stdin.clear();
					Some(ln)
				}

				_ =>
				{
					None
				}
			}
		});

		if let Some(ln) = res
		{
			return ln;
		}
	}
}
