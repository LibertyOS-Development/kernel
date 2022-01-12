// src/graphics/vga/mod.rs
//
// This is the mod.rs file for the graphics::vga module.

/*
	IMPORTS
*/

use bit_field::BitField;
use core::{fmt, fmt::Write};
use lazy_static::lazy_static;
use spin::Mutex;
use vte::{Params, Parser, Perform};
use x86_64::instructions::{interrupts, port::Port};

use crate::{font::Font, libcore::graphics::vga::{pal::Palette, std_color::Color}};

pub mod pal;
pub mod std_color;


/*
	CONSTANTS
*/

// Attribute address data register
const ATTRIBUTE_ADDRESS_DATA_REG: u16 = 0x3C0;

// Attribute data read register
const ATTRIBUTE_DATA_READ_REG: u16 = 0x3C1;

// Background color
const BACKGROUND: Color = Color::Black;

// Buffer height
const BUFFH: usize = 25;

// Buffer width
const BUFFW: usize = 80;

// CRT controller address register
const CRTC_ADDRESS_REG: u16 = 0x3D4;

// CRT controller data register
const CRTC_DATA_REG: u16 = 0x3D5;

// Digital-to-analog data register
const DAC_DATA_REG: u16 = 0x03C9;

// Digital-to-analog address write mode register (TODO: Verify that the terminology is correct)
const DAC_ADDRESS_WRITEMODE_REG: u16 = 0x3C8;

// Foreground color
const FOREGROUND: Color = Color::Yellow;

// Graphics address register
const GRAPHICS_ADDRESS_REG: u16 = 0x3CE;

// Input status register
const INPUT_STATUS_REG: u16 = 0x3Da;

// Sequencer address register
const SEQ_ADDRESS_REG: u16 = 0x3C4;

// Unprintable character replacement
const UNPRINTABLE_CHAR: u8 = 0x00;


lazy_static!
{
	pub static ref PARSER: Mutex<Parser> = Mutex::new(Parser::new());
	pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer
	{
		cur: [0; 2],
		writer: [0; 2],
		colorcode: ColorCode::new(FOREGROUND, BACKGROUND),
		buffer:
		unsafe
		{
			&mut *(0xB8000 as *mut Buffer)
		},
	});
}


// Buffer struct
#[repr(transparent)]
struct Buffer
{
	characters: [[ScreenCharacter; BUFFW]; BUFFH],
}


// ColorCode struct
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);


// ScreenCharacter struct
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenCharacter
{
	// ASCII code
	ac: u8,

	// Color code
	cc: ColorCode,
}


// Writer struct
pub struct Writer
{
	// Cursor
	cur: [usize; 2],

	// Writer
	writer: [usize; 2],

	// Color code
	colorcode: ColorCode,

	// Buffer
	buffer: &'static mut Buffer,
}

// Implementation of the ColorCode struct
impl ColorCode
{
	// New
	fn new(fg: Color, bg: Color) -> ColorCode
	{
		ColorCode((bg as u8) << 4 | (fg as u8))
	}
}


// Implementation of the Writer struct
impl Writer
{
	// Clear the next row
	fn clr_next_row(&mut self, x: usize, y: usize)
	{
		let c = ScreenCharacter
		{
			ac: b' ',
			cc: self.colorcode,
		};
	}


	// Clear the screen
	fn clr_screen(&mut self)
	{
		for y in 0..BUFFH
		{
			self.clr_next_row(0, y);
		}
	}

	// Color
	pub fn color(&self) -> (Color, Color)
	{
		// Color code
		let cc = self.colorcode.0;

		// Foreground
		let fg = std_color::fromidx(cc.get_bits(0..4) as usize);

		// Background
		let bg = std_color::fromidx(cc.get_bits(4..8) as usize);
		(fg, bg)
	}

	// Set color
	pub fn color_set(&mut self, fg: Color, bg: Color)
	{
		self.colorcode = ColorCode::new(fg, bg);
	}

	// Cursor position
	fn cpos(&self) -> (usize, usize)
	{
		(self.cur[0], self.cur[1])
	}


	// Set cursor position
	fn cpos_set(&mut self, x: usize, y: usize)
	{
		self.cur = [x, y];
		self.cur_write();
	}


	// Hide cursor
	fn cur_hide(&self)
	{
		let mut address = Port::new(CRTC_ADDRESS_REG);
		let mut data = Port::new(CRTC_DATA_REG);

		unsafe
		{
			address.write(0x0A as u8);
			data.write(0x20 as u8);
		}
	}


	// Show cursor
	fn cur_show(&self)
	{
		let mut address: Port<u8> = Port::new(CRTC_ADDRESS_REG);
		let mut data: Port<u8> = Port::new(CRTC_DATA_REG);

		// The row that the cursor begins on
		let cur_start = 13;

		// The row that the cursor ends at
		let cur_end = 14;

		unsafe
		{
			// The start register of the cursor
			address.write(0x0A);
			let b = data.read();
			data.write((b & 0xC0) | cur_start);


			// The end register of the cursor
			address.write(0x0B);
			let b = data.read();
			data.write((b & 0xE0) | cur_end);
		}
	}


	// Write cursor
	fn cur_write(&mut self)
	{
		let pos = self.cur[0] + self.cur[1] * BUFFW;
		let mut address = Port::new(CRTC_ADDRESS_REG);
		let mut data = Port::new(CRTC_DATA_REG);

		unsafe
		{
			address.write(0x0F as u8);
			data.write((pos & 0xFF) as u8);
			address.write(0x0E as u8);
			data.write(((pos >> 8) & 0xFF) as u8);
		}
	}


	// New line
	fn newln(&mut self)
	{
		if self.writer[1] < BUFFH - 1
		{
			self.writer[1] += 1;
		}
		else
		{
			for y in 1..BUFFH
			{
				for x in 0..BUFFW
				{
					unsafe
					{
						let c = core::ptr::read_volatile(&self.buffer.characters[y][x]);
						core::ptr::write_volatile(&mut self.buffer.characters[y - 1][x], c);
					}
				}
			}
			self.clr_next_row(0, BUFFH - 1);
		}
		self.writer[0] = 0;
	}


	// Set font
	pub fn setfont(&mut self, font: &Font)
	{
		let mut seq: Port<u16> = Port::new(SEQ_ADDRESS_REG);
		let mut graphics: Port<u16> = Port::new(GRAPHICS_ADDRESS_REG);
		let buffer = 0xA0000 as *mut u8;

		unsafe
		{
			// Sync reset
			seq.write(0x0100);

			// Write to plane 2
			seq.write(0x0402);

			// Sequential access
			seq.write(0x0704);

			// End reset
			seq.write(0x0300);

			// Read plane 2
			graphics.write(0x0204);

			// Disable odd/even
			graphics.write(0x0005);

			// VRAM @ 0xA0000
			graphics.write(0x0006);

			for i in 0..font.sz as usize
			{
				for j in 0..font.h as usize
				{
					// VGA offset
					let os_vga = j + i * 32 as usize;

					// Font offset
					let os_font = j + i * font.h as usize;
					buffer.add(os_vga).write_volatile(font.dat[os_font]);
				}
			}

			// Sync reset
			seq.write(0x0100);

			// Write plane 0/1
			seq.write(0x0302);

			// Even/odd access
			seq.write(0x0304);

			// End reset
			seq.write(0x0300);

			// Restore defaults
			graphics.write(0x0004);

			// Resume odd/even
			graphics.write(0x0E06);

			// VRAM @ 0xB800
			graphics.write(0x0E06);
		}
	}


	// Set palette
	pub fn setpal(&mut self, pal: Palette)
	{
		let mut address: Port<u8> = Port::new(DAC_ADDRESS_WRITEMODE_REG);
		let mut data: Port<u8> = Port::new(DAC_DATA_REG);

		for (i, (r, g, b)) in pal.color.iter().enumerate()
		{
			if i < 16
			{
				let reg = std_color::fromidx(i as usize).to_vga_reg();

				unsafe
				{
					address.write(reg);
					data.write(vga_color(*r));
					data.write(vga_color(*g));
					data.write(vga_color(*b));
				}
			}
		}
	}



	// Set writer position
	fn wpos_set(&mut self, x: usize, y: usize)
	{
		self.writer = [x, y];
	}


	// Writer position
	fn wpos(&self) -> (usize, usize)
	{
		(self.writer[0], self.writer[1])
	}


	// Write byte
	fn wrbyte(&mut self, byte: u8)
	{
		match byte
		{
			// New line
			0x0A =>
			{
				self.newln();
			},

			// Carriage return
			0x0D =>
			{
			},

			// Backspace
			0x08 =>
			{
				if self.writer[0] > 0
				{
					self.writer[0] -= 1;
					let c = ScreenCharacter
					{
						ac: b' ',
						cc: self.colorcode,
					};

					let x = self.writer[0];
					let y = self.writer[1];

					unsafe
					{
						core::ptr::write_volatile(&mut self.buffer.characters[y][x], c);
					}
				}
			},

			// Byte
			byte =>
			{
				if self.writer[0] >= BUFFW
				{
					self.newln();
				}

				let x = self.writer[0];
				let y = self.writer[1];
				let ac = if canprint(byte)
				{
					byte
				}
				else
				{
					UNPRINTABLE_CHAR
				};

				let cc = self.colorcode;
				let c = ScreenCharacter { ac, cc };

				unsafe
				{
					core::ptr::write_volatile(&mut self.buffer.characters[y][x], c);
				}

				self.writer[0] += 1;
			}
		}
	}
}


// Implementation of the Perform trait for the Writer struct
impl Perform for Writer
{
	// CSI dispatch
	fn csi_dispatch(&mut self, params: &Params, _: &[u8], _: bool, c: char)
	{
		match c
		{
			'm' =>
			{
				// Foreground
				let mut fg = FOREGROUND;

				// Background
				let mut bg = BACKGROUND;

				for param in params.iter()
				{
					match param[0]
					{
						0 =>
						{
							fg = FOREGROUND;
							bg = BACKGROUND;
						},

						30..=37 | 90..=97 =>
						{
							fg = std_color::fromansi(param[0] as u8);
						},

						40..=47 | 100..=107 =>
						{
							bg = std_color::fromansi((param[0] as u8) - 10);
						},

						_ => {},
					}
				}

				self.color_set(fg, bg);
			},

			// Cursor up
			'A' =>
			{
				let mut n = 1;
				for param in params.iter()
				{
					n = param[0] as usize;
				}

				self.writer[1] -= n;
				self.cur[1] -= n;
			},

			// Cursor down
			'B' =>
			{
				let mut n = 1;
				for param in params.iter()
				{
					n = param[0] as usize;
				}

				self.writer[1] += n;
				self.cur[1] += n;
			},

			// Cursor forward
			'C' =>
			{
				let mut n = 1;
				for param in params.iter()
				{
					n = param[0] as usize;
				}

				self.writer[0] += n;
				self.cur[0] += n;
			},

			// Cursor backwards
			'D' =>
			{
				let mut n = 1;
				for param in params.iter()
				{
					n = param[0] as usize;
				}

				self.writer[0] -= n;
				self.cur[0] -= n;
			},

			// Cursor horizontal
			'G' =>
			{
				let (_, y) = self.cpos();
				let mut x = 1;
				for param in params.iter()
				{
					// Single-indexed value
					x = param[0] as usize;
				}

				if x > BUFFW
				{
					return;
				}

				self.wpos_set(x - 1, y);
				self.cpos_set(x - 1, y);
			},

			// Move cursor
			'H' =>
			{
				let mut x = 1;
				let mut y = 1;
				for (i, param) in params.iter().enumerate()
				{
					match i
					{
						// Single-indexed value
						0 => y = param[0] as usize,

						// Single-indexed value
						1 => x = param[0] as usize,

						_ => break,
					};
				}

				if x > BUFFW || y > BUFFH
				{
					return;
				}

				self.wpos_set(x - 1, y - 1);
				self.cpos_set(x - 1, y - 1);
			},

			// Erase (for display)
			'J' =>
			{
				let mut n = 0;
				for param in params.iter()
				{
					n = param[0] as usize;
				}

				match n
				{
					2 => self.clr_screen(),
					_ => return,
				}

				self.wpos_set(0, 0);
				self.cpos_set(0, 0);
			},

			// Erase (for line)
			'K' =>
			{
				let (x, y) = self.cpos();
				let mut n = 0;
				for param in params.iter()
				{
					n = param[0] as usize;
				}

				match n
				{
					0 => self.clr_next_row(x, y),
					1 => return,
					2 => self.clr_next_row(0, y),
					_ => return,
				}

				self.wpos_set(x, y);
				self.cpos_set(x, y);
			},

			// Enable
			'h' =>
			{
				for param in params.iter()
				{
					match param[0]
					{
						25 => self.cur_show(),
						_ => return,
					}
				}
			},

			// Disable
			'l' =>
			{
				for param in params.iter()
				{
					match param[0]
					{
						25 => self.cur_hide(),
						_ => return,
					}
				}
			},

			_ => {},
		}
	}

	// Execute
	fn execute(&mut self, byte: u8)
	{
		self.wrbyte(byte);
	}


	// Print
	fn print(&mut self, c: char)
	{
		self.wrbyte(c as u8);
	}
}


// Implementation of the Write trait for Writer
impl fmt::Write for Writer
{
	// Write string
	fn write_str(&mut self, s: &str) -> fmt::Result
	{
		let mut parser = PARSER.lock();
		for byte in s.bytes()
		{
			parser.advance(self, byte);
		}

		let (x, y) = self.wpos();
		self.cpos_set(x, y);
		Ok(())
	}
}

// Can print
pub fn canprint(c: u8) -> bool
{
	matches!(c, 0x20..=0x7E | 0x08 | 0x0A | 0x0D | 0x7F..=0xFF)
}

// Columns
pub fn col() -> usize
{
	BUFFW
}


// Color function
pub fn color() -> (Color, Color)
{
	interrupts::without_interrupts(||
	{
		WRITER.lock().color()
	})
}


// Get attribute control register
fn get_attributectl_reg(idx: u8) -> u8
{
	interrupts::without_interrupts(||
	{
		// Input status register
		let mut isr: Port<u8> = Port::new(INPUT_STATUS_REG);

		// Address
		let mut address: Port<u8> = Port::new(ATTRIBUTE_ADDRESS_DATA_REG);

		// Data
		let mut data: Port<u8> = Port::new(ATTRIBUTE_DATA_READ_REG);

		// Set the palette address source bit
		let idx = idx| 0x20;

		unsafe
		{
			// Reset to address mode
			isr.read();
			let tmp = address.read();
			address.write(idx);
			let res = data.read();
			address.write(tmp);
			res
		}
	})
}


// Print formatting
pub fn printfmt(args: fmt::Arguments)
{
	interrupts::without_interrupts(||
	{
		WRITER.lock().write_fmt(args)
			.expect("[ERR] COULD NOT PRINT TO VGA");
	});
}


// Set attribute control register
fn set_attributectl_reg(idx: u8, val: u8)
{
	interrupts::without_interrupts(||
	{
		// Input status register
		let mut isr: Port<u8> = Port::new(INPUT_STATUS_REG);

		// Address
		let mut address: Port<u8> = Port::new(ATTRIBUTE_ADDRESS_DATA_REG);

		unsafe
		{
			// Reset to address mode
			isr.read();
			let tmp = address.read();
			address.write(idx);
			address.write(val);
			address.write(tmp);
		}
	})
}


// Set color
pub fn setcolor(fg: Color, bg: Color)
{
	interrupts::without_interrupts(||
	{
		WRITER.lock().color_set(fg, bg)
	})
}


// Set font
pub fn setfont(font: &Font)
{
	interrupts::without_interrupts(||
	{
		WRITER.lock().setfont(font);
	})
}


// Set palette
pub fn setpal(palette: Palette)
{
	interrupts::without_interrupts(||
	{
		WRITER.lock().setpal(palette)
	})
}


// Set underline location
fn set_ul_loc(location: u8)
{
	interrupts::without_interrupts(||
	{
		let mut address: Port<u8> = Port::new(CRTC_ADDRESS_REG);
		let mut data: Port<u8> = Port::new(CRTC_DATA_REG);

		unsafe
		{
			// Register for the underline location
			address.write(0x14);
			data.write(location);
		}
	})
}


// Rows
pub fn row() -> usize
{
	BUFFH
}


// VGA color (converts 8-bit color to 6-bit color)
fn vga_color(color: u8) -> u8
{
	color >> 2
}


// Initialization
pub fn init()
{
	// Map palette registers to color registers
	set_attributectl_reg(0x0, 0x00);
	set_attributectl_reg(0x1, 0x01);
	set_attributectl_reg(0x2, 0x02);
	set_attributectl_reg(0x3, 0x03);
	set_attributectl_reg(0x4, 0x04);
	set_attributectl_reg(0x5, 0x05);
	set_attributectl_reg(0x6, 0x14);
	set_attributectl_reg(0x7, 0x07);
	set_attributectl_reg(0x8, 0x38);
	set_attributectl_reg(0x9, 0x39);
	set_attributectl_reg(0xA, 0x3A);
	set_attributectl_reg(0xB, 0x3B);
	set_attributectl_reg(0xC, 0x3C);
	set_attributectl_reg(0xD, 0x3D);
	set_attributectl_reg(0xE, 0x3E);
	set_attributectl_reg(0xF, 0x3F);

	setpal(Palette::def());

	// Disable blinking

	// Attribute mode control register
	let reg = 0x10;
	let mut attribute = get_attributectl_reg(reg);

	// Clear the bit that enabled blinking
	attribute.set_bit(3, false);
	set_attributectl_reg(reg, attribute);

	// Disable underline
	set_ul_loc(0x1F);

	WRITER.lock().clr_screen();
}
