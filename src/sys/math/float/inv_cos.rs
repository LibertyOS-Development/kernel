// src/math/float/inv_cos.rs
//
// This module adds the ability to calculate the inverted cosine (arcosine) of a floating point.

use core::f32::consts::PI;
use crate::sys::math::float::fl::FL32;

impl FL32
{
	// This will calculate the approximate acrocine of a given float:
	pub fn inv_cos(self) -> Self
	{
		if self > 0.0
		{
			((Self::ONE - self * self).sqrt() / self).inv_tan()
		}
		else if self == 0.0
		{
			Self(PI / 2.0)
		}
		else
		{
			((Self::ONE - self * self).sqrt() / self).inv_tan() + PI
		}
	}
}
