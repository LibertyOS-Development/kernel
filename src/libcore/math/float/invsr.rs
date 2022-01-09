// src/libcore/math/float/invsr.rs
//
// Calculate the inverted form of the square-root of a floating-point value.


use crate::libcore::math::float::fl::FL32;


impl FL32
{
	// Calculate the inverted form of the square-root of a specified value
	pub fn invsr(self) -> Self
	{
		Self::conv_from_bits(0x5f37_5a86 - (self.conv_to_bits() >> 1))
	}
}
