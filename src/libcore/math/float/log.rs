// src/math/float/log.rs
//
// This module adds the ability to calculate the logarithm of a single-precision float.

use crate::libcore::math::float::fl::FL32;

impl FL32
{
	// This approximates the logarithm of a float:
	pub fn log(self, base: Self) -> Self
	{
		(Self:: ONE / base.nlog()) * self.nlog()
	}
}
