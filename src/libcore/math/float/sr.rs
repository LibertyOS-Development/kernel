// src/math/float/sr.rs
//
// This module adds the ability to calculate the square root of a given float.

use crate::libcore::math::float::fl::FL32;

impl FL32
{
	// This will approximate the square root of a given float:
	// NOTE: Standard deviation is five percent (5%).
	pub fn sr(self) -> Self
	{
		if self >= Self::ZERO
		{
			Self::conv_from_bits((self.conv_to_bits() + 0x3f80_0000) >> 1)
		}
		else
		{
			Self::NAN
		}
	}
}
