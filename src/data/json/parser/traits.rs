pub trait Pos: core::ops::Sub<Self, Output = i32> + Copy
{
	fn idx(&self) -> u32;
	fn ln(&self) -> u32;
	fn col(&self) -> u32;
}

pub trait Err
{
	type Pos;
	fn cause(&self) -> &[(Self::Pos, &'static str)];
	fn new_cause(self, pos: Self::Pos, cause: &'static str) -> Self;
}

pub trait Input: Default
{
	type Pos: Pos;
	type Err: Err<Pos = Self::Pos>;

	fn next(&self, pos: Self::Pos) -> Result<(char, Self::Pos), Self::Err>;

	fn next_range(&self, start: Self::Pos, count: u32) -> Result<(&str, Self::Pos), Self::Err>;
	fn err_at(&self, pos: Self::Pos, cause: &'static str) -> Self::Err;
	fn end(&self, pos: Self::Pos) -> bool;
}

pub type ResultOf<I, O> = Result<(O, <I as Input>::Pos), <I as Input>::Err>;
