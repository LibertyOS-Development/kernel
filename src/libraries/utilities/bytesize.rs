use core::fmt;

#[repr(transparent)]
pub struct ByteSize(usize);

impl ByteSize
{
	pub const fn new(val: usize) -> ByteSize
	{
		ByteSize(val)
	}
}
