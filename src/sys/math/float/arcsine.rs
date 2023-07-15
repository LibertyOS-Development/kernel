// src/math/arcsine.rs
//
// Arcsine function for floating-point values.

use crate::sys::math::float::fl::FL32;


impl FL32
{
	// Calculate the arcsine of the specified value
	pub fn arcsine(self) -> Self
	{
		(self * (Self::ONE - self * self).inv_sqrt()).inv_tan()
	}
}
