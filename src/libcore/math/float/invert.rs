// src/math/float/invert.rs
//
// This module adds the ability to calculate the inverted form of a specific floating point.

use crate::libcore::math::float::fl::FL32;

impl FL32
{
	pub fn invert(self) -> Self
	{
		Self(f32::from_bits(0x7f00_0000 - self.0.to_bits()))
	}
}
