// src/math/float/cpsign.rs
//
// Copy the sign of a given number to another number.

use crate::libcore::math::float::fl::{FL32, SIGN_M};

impl FL32
{
	// Return a number, comprised of self's size, with the sign of the "sign" variable.
	pub fn cpsign(self, sign: Self) -> Self
	{
		let srcbits = sign.conv_to_bits();
		let srcsign = srcbits & SIGN_M;
		let signless_dest_bits = self.conv_to_bits() & !SIGN_M;
		Self::conv_from_bits(signless_dest_bits | srcsign)
	}
}
