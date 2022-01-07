// src/math/float/sine.rs
//
// Calculate sine for floats.

use core::f32::consts::PI;
use crate::libcore::math::float::fl::FL32;


impl FL32
{
	// Calculate sine.
	// NOTE: Margin of error is around 0.002%.
	pub fn sine(self) -> Self
	{
		(self - PI / 2.0).cosine()
	}
}
