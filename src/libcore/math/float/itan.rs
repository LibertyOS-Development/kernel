// src/math/float/itan.rs
//
// This module adds the ability to calculate the inverse tangent (arctangent) of floats.

use core::f32::consts::FRAC_PI_2;
use crate::libcore::math::float::fl::FL32;

impl FL32
{
	// This calculates the arctangent of a float:
	// NOTE: The margin of error is roughly 0.02%.
	pub fn itan(self) -> Self
	{
		FRAC_PI_2 * self.nitan()
	}

	// This calculates the normalized arctangent of a float:
	// NOTE: The margin of error is roughly 16.20%.
	pub fn nitan(self) -> Self
	{
		const SIGN_M: u32 = 0x8000_0000;
		const B: f32 = 0.596_227;
		// This removes the bit with the sign:
		let uxs = SIGN_M & self.conv_to_bits();
		// This will calculate the arctangent of the first quadrant:
		let bxa = (B * self).absval();
		let n = bxa + self * self;
		let itan1q = n / (1.0 + bxa + n);
		// This will replace the bit with the sign, then convert the result to a float:
		Self::conv_from_bits(uxs | itan1q.conv_to_bits())
	}
}
