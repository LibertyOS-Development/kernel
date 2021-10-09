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
	DarkGray = 8,
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
	fn new(foreground: Color, background: Color) -> ColorCode
	{
		ColorCode((background as u8) << 4 | (foreground as u8))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar
{
	asciichar: u8,
	colorcode: ColorCode,
}

const BUFF_H: usize = 25;
const BUFF_W: usize = 80;

#[repr(transparent)]
struct Buffer
{
	chars: [[ScreenChar; BUFF_W]; BUFF_H],
}

pub struct Writer
{
	colpos: usize,
	colorcode: ColorCode,
	buffer: &'static mut Buffer,
}

impl Writer
{
	pub fn writebyte(&mut self, byte: u8)
	{
		match byte
		{
			b'\n' => self.newln(),
			byte =>
			{
				if self.colpos >= BUFF_W
				{
					self.newln();
				}
				let row = BUFF_H - 1;
				let col = self.colpos;
				let colorcode = self.colorcode;
				self.buffer.chars[row][col] = ScreenChar
				{
					asciichar: byte,
					colorcode,
				};
				self.colpos += 1;
			}
		}
	}
	fn newln(&mut self) 
	{
		for row in 1..BUFF_H
		{
			for col in 0..BUFF_W
			{
				let char = self.buffer.chars[row][col].read();
				self.buffer.chars[row - 1][col].write(char);
			}
		}
		self.clrrow(BUFF_H - 1);
		self.colpos = 0;
	}
	fn clrrow(&mut self, row: usize) {}
	pub fn writestr(&mut self, s: &str)
	{
		for byte in s.bytes()
		{
			match byte
			{
				0x20..=0x7e | b'\n' => self.writebyte(byte),
				_ => self.writebyte(0xfe),
			}
		}
	}
}

pub fn print_whatever()
{
	let mut writer = Writer
	{
		colpos: 0,
		colorcode: ColorCode::new(Color::Yellow, Color::Black),
		buffer: unsafe
		{
			&mut *(0xb8000 as *mut Buffer)
		},
	};
	writer.writebyte(b'H');
	writer.writestr("owdy ");
	writer.writestr("pardner!");
}


use core::fmt;

impl fmt::Write for Writer
{
	fn write_str(&mut self, s: &str) -> fmt::Result
	{
		self.write_str(s);
		Ok(())
	}
}
