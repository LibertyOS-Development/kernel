// src/math/units/bparse.rs
//
// The math::units::bparse module is designed to help parse bytes.

use core::convert::TryInto;

fn aligndown(val: usize, align: usize) -> usize
{
	val & !(align - 1)
}

fn alignup(val: usize, align: usize) -> usize
{
	aligndown(val + align - 1, align)
}

#[derive(Debug, PartialEq)]
pub enum ByteParseErr
{
	NotLongEnough,
}

pub struct ByteParse<'a>
{
	buff: &'a [u8],
	curr: usize,
}

impl<'a> ByteParse<'a>
{
	pub fn new(buff: &'a [u8]) -> ByteParse<'a>
	{
		ByteParse
		{
			buff,
			curr: 0
		}
	}

	pub fn rem(&self) -> &[u8]
	{
		&self.buff[self.curr..]
	}

	pub fn remlen(&self) -> usize
	{
		self.buff.len() - self.curr
	}

	pub fn skip(&mut self, len: usize) -> Result<(), ByteParseErr>
	{
		if self.curr + len > self.buff.len()
		{
			return Err(ByteParseErr::NotLongEnough);
		}
		self.curr += len;
		Ok(())
	}

	pub fn skip_until_align(&mut self, align: usize) -> Result<(), ByteParseErr>
	{
		let next = alignup(self.curr, align);
		if next > self.buff.len()
		{
			return Err(ByteParseErr::NotLongEnough);
		}
		self.curr = next;
		Ok(())
	}

	pub fn consbyte(&mut self, len: usize) -> Result<&'a [u8], ByteParseErr>
	{
		if self.curr + len > self.buff.len()
		{
			return Err(ByteParseErr::NotLongEnough);
		}
		self.curr += len;
		Ok(&self.buff[self.curr - len..self.curr])
	}

	pub fn cons_leu16(&mut self) -> Result<u16, ByteParseErr>
	{
		if self.remlen() < 2
		{
			return Err(ByteParseErr::NotLongEnough);
		}

		let val = u16::from_le_bytes(self.buff[self.curr..self.curr + 2].try_into().unwrap());
		self.curr += 2;
		Ok(val)
	}

	pub fn cons_leu32(&mut self) -> Result<u32, ByteParseErr>
	{
		if self.remlen() < 4
		{
			return Err(ByteParseErr::NotLongEnough);
		}

		let val = u32::from_le_bytes(self.buff[self.curr..self.curr + 4].try_into().unwrap());
		self.curr += 4;
		Ok(val)
	}

	pub fn cons_leu64(&mut self) -> Result<u64, ByteParseErr>
	{
		if self.remlen() < 8
		{
			return Err(ByteParseErr::NotLongEnough);
		}

		let val = u64::from_le_bytes(self.buff[self.curr..self.curr + 8].try_into().unwrap());
		self.curr += 8;
		Ok(val)
	}

	pub fn cons_lei32(&mut self) -> Result<i32, ByteParseErr>
	{
		if self.remlen() < 4
		{
			return Err(ByteParseErr::NotLongEnough);
		}
		let val = i32::from_le_bytes(self.buff[self.curr..self.curr + 4].try_into().unwrap());
		self.curr += 4;
		Ok(val)
	}
}
