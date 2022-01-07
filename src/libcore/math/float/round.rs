// src/math/float/round.rs
//
// Rounding for floats.

use crate::libcore::math::float::fl::FL32;

impl FL32
{
	// Take specified input and return the nearest integer.
	pub fn round(self) -> Self
	{
		Self(((self.0 + Self(0.5).cpsign(self).0) as i32) as f32)
	}
}
