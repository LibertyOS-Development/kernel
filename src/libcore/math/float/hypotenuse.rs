// src/math/float/hypotenuse.rs
//
// This module adds the ability to calculate the hypotenuse of a given triangle.

use crate::libcore::math::float::fl::FL32;

impl FL32
{
	// This calculates the hypotenuse's length:
	pub fn hypotenuse(self, rhs: Self) -> Self
	{
		(self * self + rhs * rhs).sr()
	}
}
