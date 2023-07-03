use crate::data::json::parser::traits::{Err, Pos};
use alloc::{fmt::Formatter, vec::Vec};

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct SimplePos
{
	pub idx: u32,
	pub ln: u32,
	pub col: u32,
}


impl SimplePos
{
	pub fn next(&self, c: char) -> Self
	{
		let newln = c == '\n';

		Self
		{
			idx: self.idx + 1,

			ln: if newln
			{
				self.ln + 1
			}
			else
			{
				self.ln
			},

			col: if newln
			{
				0
			}
			else
			{
				self.col + 1
			},
		}
	}
}


impl Pos for SimplePos
{
	fn idx(&self) -> u32
	{
		self.idx
	}

	fn ln(&self) -> u32
	{
		self.ln
	}

	fn col(&self) -> u32
	{
		self.col
	}
}


impl core::ops::Sub<Self> for SimplePos
{
	type Output = i32;

	fn sub(self, rhs: SimplePos) -> Self::Output
	{
		if self.idx > rhs.idx
		{
			(self.idx - rhs.idx) as i32
		}
		else
		{
			-((rhs.idx - self.idx) as i32)
		}
	}
}


// This breaks the kernel. :(
// #[derive(Debug, PartialEq, Eq)]
pub struct SimpleErr
{
	pub causes: Vec<(SimplePos, &'static str)>,
}


impl core::fmt::Debug for SimpleErr
{
	fn fmt(&self, _f: &mut Formatter<'_>) -> core::fmt::Result
	{
		Ok(())
	}
}

impl Err for SimpleErr
{
	type Pos = SimplePos;

	fn cause(&self) -> &[(Self::Pos, &'static str)]
	{
		&self.causes[..]
	}

	fn new_cause(self, pos: Self::Pos, cause: &'static str) -> Self
	{
		let mut causes = self.causes;
		causes.push((pos, cause));
		Self
		{
			causes
		}
	}
}
