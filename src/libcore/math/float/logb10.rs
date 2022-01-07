// src/math/float/logb10.rs
//
// Calculate the logarithm of a float (in base-10).

use core::f32::consts::LOG10_E;
use crate::libcore::math::float::fl::FL32;

impl FL32
{
	// Calculate the logarithm of a float (in base-10).
	pub fn logb10(self) -> Self
	{
		self.nlog() * LOG10_E
	}
}
