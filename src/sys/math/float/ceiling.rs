// src/math/ceiling.rs
//
// Calculate ceiling for floating-point values.

use crate::sys::math::float::fl::FL32;


impl FL32
{
	// Calculates the ceiling of a specific value
	pub fn ceiling(self) -> Self
	{
		-(-self).floor()
	}
}
