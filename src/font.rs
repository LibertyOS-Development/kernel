// font.rs

use alloc::vec::Vec;

#[derive(Clone)]
pub struct Font
{
	pub h: u8,
	pub sz: u16,
	pub dat: Vec<u8>,
}


pub fn frombytes(buf: &[u8]) -> Result<Font, ()>
{
	if buf.len() < 4 || buf[0] != 0x36 || buf[1] != 0x04
	{
		return Err(());
	}
	let mode = buf[2];
	let h = buf[3];
	let sz = match mode
	{
		0 | 2 => 256,
		1 | 3 => 512,
		_ => return Err(()),
	};

	let n = (4 + sz * h as u16) as usize;
	if buf.len() < n
	{
		return Err(());
	}
	let dat = buf[4..n].to_vec();

	Ok(Font
	{
		h,
		sz,
		dat
	})
}
