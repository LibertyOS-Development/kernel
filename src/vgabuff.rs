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
	fn new(fground: Color, bground: Color) -> ColorCode
	{
		ColorCode((bground as u8) << 4 | (fground as u8))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar
{
	asciichar: u8,
	colorcode: ColorCode,
}

const BUFFH: usize = 25;
const BUFFW: usize = 80;

use volatile::Volatile;

struct Buff
{
	chars: [[Volatile<ScreenChar>; BUFFW]; BUFFH],
}

pub struct Write
{
	colpos: usize,
	colorcode: ColorCode,
	buff: &'static mut Buff,
}

impl Write
{
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
				self.buff.chars[row][col].write(ScreenChar
				{
					asciichar: byte,
					colorcode,
				});
				self.colpos += 1;
			}
		}
	}

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

	fn newln(&mut self) 
	{
		for row in 1..BUFFH
		{
			for col in 0..BUFFW
			{
				let char = self.buff.chars[row][col].read();
				self.buff.chars[row - 1][col].write(char);
			}
		}
		self.clrrow(BUFFH - 1);
		self.colpos = 0;
	}
	fn clrrow(&mut self, row: usize) {}
}



