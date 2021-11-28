// LIBERTYOS: src/vgabuff.rs
//
// This module provides the ability for the kernel to write text to the VGA text-buffer,
// use colored text, provide required features for various macros to function correctly, print lines, create lines, etc.
//
// DEPRECATION STATUS:
// This module has been available since the beginning of the kernel's development, and there are no plans to deprecate this module.


use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;


// This is a globally-available instance of "WRITER", which is used by the print!/println!
// macros. In addition to being used by the aforementioned macros, this instance can be used to print to the VGA text-buffer.
lazy_static!
{
	pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer
	{
		colpos: 0,
		colorcode: ColorCode::new(Color::Red, Color::Black),
		buffer: unsafe
		{
			&mut *(0xb8000 as *mut Buffer)
		},
	});
}

// This provides the kernel with a simple color palette for VGA text.
// LightGray and DarkGray (numbers 7 and 8, respectively) have versions that are
// identical in purpose/functionality. The alternate versions in question use the Britsh form of the word "gray".

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color
{
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Brown = 6,

	LightGray = 7,
//	LightGrey = 7,

	DarkGray = 8,
//	DarkGrey = 8,

	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	Pink = 13,
	Yellow = 14,
	White = 15,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);
impl ColorCode
{
	// This creates a color-code, with a specific foreground and background colors.
	fn new(foreground: Color, background: Color) -> ColorCode
	{
		ColorCode((background as u8) << 4 | (foreground as u8))
	}
}


// A single screen character, consisting of an ASCII character and a color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar
{
	asciichar: u8,
	colorcode: ColorCode,
}

// This sets the height of the text-buffer.
const BUFFH: usize = 25;
// This sets the width of the text-buffer.
const BUFFW: usize = 80;


// This structure represents the VGA text-buffer.
#[repr(transparent)]
struct Buffer
{
	 chars: [[Volatile<ScreenChar>; BUFFW]; BUFFH],
}


// This is a type that allows for writing ASCII bytes/strings to an existing "Buffer".
//
// Lines are wrapped when the width exceeds the length of BUFFW.
pub struct Writer
{
	colpos: usize,
	colorcode: ColorCode,
	buffer: &'static mut Buffer,
}

impl Writer
{
	// Write a single ASCII byte to the buffer.
	//
	// Lines are wrapped when the width of the buffer matches BUFFW.
	// Supports the '\n' character.

	pub fn writebyte(&mut self, byte: u8)
	{
		match byte
		{
			b'\n' => self.newln(),
			byte =>
			{
				if self.colpos >= BUFFW
				{
					self.newln();
				}

				let row = BUFFH - 1;
				let col = self.colpos;

				let colorcode = self.colorcode;
				self.buffer.chars[row][col].write(ScreenChar
				{
					asciichar: byte,
					colorcode,
				});
			}
		}
	}

	// Write an ASCII string to the buffer.
	//
	// Lines are wrapped when the width of the buffer matches the BUFFW.
	// Supports using the '\n' character.
	// Does NOT support using non-ASCII charcters (VGA text-buffer can only use ASCII).

	fn write_str(&mut self, s: &str)
	{
		for byte in s.bytes()
		{
			match byte
			{
				// Supported ASCII byte/newline ('\n')
				0x20..=0x7e | b'\n' => self.writebyte(byte),
				// Unsupported characters
				_ => self.writebyte(0xfe),
			}
		}
	}


	// Clears the last row, shifts all lines up by one.

	fn newln(&mut self)
	{
		for row in 1..BUFFH
		{
			for col in 0..BUFFW
			{
				let char = self.buffer.chars[row][col].read();
				self.buffer.chars[row - 1][col].write(char);
			}
		}
		self.clear_row(BUFFH - 1);
		self.colpos = 0;
	}


	// Clears a row by writing a row of blank characters to the buffer.
	fn clear_row(&mut self, row: usize)
	{
		let blank = ScreenChar
		{
			asciichar: b' ',
			colorcode: self.colorcode,
		};
		for col in 0..BUFFW
		{
			self.buffer.chars[row][col].write(blank);
		}
	}
}


impl fmt::Write for Writer
{
	fn write_str(&mut self, s: &str) -> fmt::Result
	{
		self.write_str(s);
		Ok(())
	}
}


// MACROS

// PRINT:
// This macro is almost identical to the print! macro (from the standard library), except this macro prints to the VGA text-buffer.
#[macro_export]
macro_rules! print
{
	($($arg:tt)*) => ($crate::vgabuff::_print(format_args!($($arg)*)));
}


// PRINTLN:
// This macro is almost identical to the println! macro (from the standard library), except this macro prints the VGA text-buffer.
#[macro_export]
macro_rules! println
{
	() => ($crate::print!("\n"));
	($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}


// This function prints the provided string to the VGA text-buffer, using the WRITE instance.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments)
{
	use core::fmt::Write;
	use x86_64::instructions::interrupts;
	interrupts::without_interrupts(|| { WRITER.lock().write_fmt(args).unwrap(); });
}


// TESTING
//
//
//
// TEST-CASE #1: test_println_simple
//
// This test-case prints a simple message to the VGA buffer.

#[test_case]
fn test_println_simple()
{
	println!("TEST_PRINTLN_SIMPLE OUTPUT");
}


// TEST-CASE #2: test_println_many
//
// This test-case prints many lines to the VGA buffer.

#[test_case]
fn test_println_many()
{
	for _ in 0..200
	{
		println!("TEST_PRINTLN_MANY OUTPUT");
	}
}


// TEST-CASE #3: test_println_output
//
// This test-case verifies that printed text is actually being printed onto the display.

#[test_case]
fn test_println_output()
{
	use core::fmt::Writer;
	use x86_64::instructions::interrupts;

	let s = "[INFO] TEST";
	interrupts::without_interrupts(||
	{
		let mut write = WRITER.lock();
		writeln!(write, "\n{}", s).expect("[ERR] WRITELN FAILURE");
		for (i, c) in s.chars().enumerate()
		{
			let screenchar = write.buffer.chars[BUFFH - 2][i].read();
			assert_eq!(char::from(screenchar.asciichar), c);
		}
	});
}
