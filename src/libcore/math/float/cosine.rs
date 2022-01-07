// src/math/float/cosine.rs
//
// Core functionality of the "consine" mathematical function.

use core::f32::consts::FRAC_1_PI;
use crate::libcore::math::float::fl::FL32;

impl FL32
{
	// Calculate the cosine of a float.
	// NOTE: The margin of error is roughly 0.02%.
	pub fn cosine(self) -> Self
	{
		let mut x = self;
		x *= FRAC_1_PI / 2.0;
		x -= 0.25 + (x + 0.25).floor().0;
		x *= 16.0 * (x.absval() - 0.5);
		x += 0.225 * x * (x.absval() - 1.0);
		x
	}
}
