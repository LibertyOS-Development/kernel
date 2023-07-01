// src/graphics/vga/pal.rs
//
// Basic code to implement color palettes.

/*
	IMPORTS
*/

use alloc::vec::Vec;
use core::convert::TryInto;


pub struct Palette
{
	pub color: [(u8, u8, u8); 16]
}

impl Palette
{
	pub fn def() -> Palette
	{
		Palette
		{
			color:
			[
				// BLACK
				(0x00, 0x00, 0x00),

				// BLUE
				(0x00, 0x00, 0x80),

				// GREEN
				(0x00, 0x80, 0x00),

				// CYAN
				(0x00, 0x80, 0x80),

				// RED
				(0x80, 0x00, 0x00),

				// MAGENTA
				(0x80, 0x00, 0x80),

				// BROWN
				(0x80, 0x80, 0x00),

				// LIGHT GRAY
				(0xC0, 0xC0, 0xC0),

				// GRAY
				(0x80, 0x80, 0x80),

				// LIGHT BLUE
				(0x00, 0x00, 0xFF),

				// LIGHT GREEN
				(0x00, 0xFF, 0x00),

				// LIGHT CYAN
				(0x00, 0xFF, 0xFF),

				// LIGHT RED
				(0xFF, 0x00, 0x00),

				// PINK
				(0xFF, 0x00, 0xFF),

				// YELLOW
				(0xFF, 0xFF, 0x00),

				// WHITE
				(0xFF, 0xFF, 0xFF),
			]
		}
	}
}


// Parse/interpret palettes from CSV files:
pub fn parse_csv(s: &str) -> Result<Palette, ()>
{
	let color: Vec<_> = s.split('\n').filter_map(|line|
	{
		// Remove any comments:
		let ln = line.split('#').next().unwrap();
		let color: Vec<u8> = ln.split(',').filter_map(|value|
		{
			let radix = if value.contains("0x")
			{
				16
			}
			else
			{
				10
			};
		let value = value.trim().trim_start_matches("0x");
		u8::from_str_radix(value, radix).ok()
	}).collect();

	// RGB
	if color.len() == 3
	{
		Some((color[0], color[1], color[2]))
	}
	else
	{
		None
	}}).collect();

	// 16 COLOR ARRAY
	if let Ok(color) = color.try_into()
	{
		Ok(Palette
		{
			color
		})
	}
	else
	{
		Err(())
	}
}
