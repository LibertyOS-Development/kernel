// src/math/float/tangent.rs
//
// Calculate tangents for floats.

use crate::libcore::math::float::fl::FL32;

impl FL32
{
	pub fn tangent(self) -> Self
	{
		self.sine() / self.cosine()
	}
}
