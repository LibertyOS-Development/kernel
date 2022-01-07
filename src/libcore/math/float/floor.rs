// src/math/float/floor.rs
//
// This module adds the ability to calculate the approximate floor for a floating point.

use crate::libcore::math::float::fl::FL32;

impl FL32
{
	// This will return the greatest integer that is <= a given float:
	pub fn floor(self) -> Self
	{
		let mut result = (self.0 as i32) as f32;
		if self.0 < result
		{
			result -= 1.0;
		}
		Self(result)
	}
}
