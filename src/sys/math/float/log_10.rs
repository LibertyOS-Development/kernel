// src/math/float/log_10.rs
//
// Calculate the logarithm of a float (in base-10).

use core::f32::consts::LOG10_E;
use crate::sys::math::float::fl::FL32;

impl FL32
{
	// Calculate the logarithm of a float (in base-10).
	pub fn log_10(self) -> Self
	{
		self.ln() * LOG10_E
	}
}
