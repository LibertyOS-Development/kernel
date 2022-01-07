// src/math/float/logb2.rs
//
// Provides the kernel with the ability to calculate the logarithm of a single-precision float (base-2).

use core::f32::consts::LOG2_E;
use crate::libcore::math::float::fl::FL32;

impl FL32
{
	// Calculates the approximate logarithm of the number in question (base-2)
	pub fn logb2(self) -> Self
	{
		self.nlog() * LOG2_E
	}
}
