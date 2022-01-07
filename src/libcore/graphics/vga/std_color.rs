// src/graphics/vga/std_color.rs
//
// Standard color palette (VGA).

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
	Gray = 8,
	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	Pink = 13,
	Yellow = 14,
	White = 15,
}

const COLOR: [Color; 16] = [
	Color::Black,
	Color::Blue,
	Color::Green,
	Color::Cyan,
	Color::Red,
	Color::Magenta,
	Color::Brown,
	Color::LightGray,
	Color::Gray,
	Color::LightBlue,
	Color::LightGreen,
	Color::LightCyan,
	Color::LightRed,
	Color::Pink,
	Color::Yellow,
	Color::White,
];

pub fn color() -> [Color; 16]
{
	COLOR
}

pub fn fromidx(index: usize) -> Color
{
	COLOR[index]
}

pub fn fromansi(code: u8) -> Color
{
	match code
	{
		30 => Color::Black,
		31 => Color::Red,
		32 => Color::Green,
		33 => Color::Brown,
		34 => Color::Blue,
		35 => Color::Magenta,
		36 => Color::Cyan,
		37 => Color::LightGray,
		90 => Color::Gray,
		91 => Color::LightRed,
		92 => Color::LightGreen,
		93 => Color::Yellow,
		94 => Color::LightBlue,
		95 => Color::Pink,
		96 => Color::LightCyan,
		97 => Color::White,

		// ERROR
		_ => Color::Black,
	}
}

impl Color
{
	pub fn tovgareg(&self) -> u8
	{
		match self
		{
			Color::Black => 0x00,
			Color::Blue => 0x01,
			Color::Green => 0x02,
			Color::Cyan => 0x03,
			Color::Red => 0x04,
			Color::Magenta => 0x05,
			Color::LightGray => 0x07,
			Color::Brown => 0x14,
			Color::Gray => 0x38,
			Color::LightBlue => 0x39,
			Color::LightGreen => 0x3A,
			Color::LightCyan => 0x3B,
			Color::LightRed => 0x3C,
			Color::Pink => 0x3D,
			Color::Yellow => 0x3E,
			Color::White => 0x3F,
		}
	}
}
