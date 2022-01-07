// src/math/float/absval.rs
//
// This module adds the ability to calculate the absolute value of a specific single-precision float.

use crate::libcore::math::float::fl::{FL32, SIGN_M};

impl FL32
{
	pub fn absval(self) -> Self
	{
		Self::conv_from_bits(self.conv_to_bits() & !SIGN_M)
	}
}
