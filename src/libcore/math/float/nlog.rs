// src/math/float/nlog.rs
//
// This module adds the ability to approximate the natural log of single-precision floats.

use crate::libcore::math::float::fl::{EXP_M, FL32};
use core::f32::consts::LN_2;

impl FL32
{
	// This approximates the natural logarithm (nlog) of a given value:
	pub fn nlog(self) -> Self
	{
		if (self - Self::ONE).absval() < f32::EPSILON
		{
			return Self::ZERO;
		}
		let x_lt_1 = self < 1.0;
		let x_work = if x_lt_1
		{
			self.invert()
		}
		else
		{
			self
		};
		// This determines the value of the exponent:
		let b2exp = x_work.ext_exp_val() as u32;
		let divisor = f32::from_bits(x_work.conv_to_bits() & EXP_M);
		let x_work = x_work / divisor;
		let nlog_1to2_pn = -1.741_793_9 + (2.821_202_6 + (-1.469_956_8 + (0.447_179_55 - 0.056_570_851 * x_work) * x_work) * x_work) * x_work;

		// This evaluates the following expression: nlog(2) * n + nlog(y)
		let result = (b2exp as f32) * LN_2 + nlog_1to2_pn;
		if x_lt_1
		{
			-result
		}
		else
		{
			result
		}
	}
}

