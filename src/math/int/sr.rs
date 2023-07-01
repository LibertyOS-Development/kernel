// src/math/int/sr.rs
//
// This is a simple library to calculate square-roots from integers.

pub trait IntSqRoot
{
	fn intsqroot(&self) -> Self
	where
		Self: Sized,
	{
		self.intsqroot_check()
			.expect("[ERR] CANNOT CALCULATE THE SQUARE ROOT OF A NEGATIVE INTEGER")
	}

	// Calculate the square root, or return "None" in the event of the integer being negative.
	fn intsqroot_check(&self) -> Option<Self>
	where
		Self: Sized;
}


impl<T: num_traits::PrimInt> IntSqRoot for T
{
	fn intsqroot_check(&self) -> Option<Self>
	{
		use core::cmp::Ordering;
		match self.cmp(&T::zero())
		{
			Ordering::Less => return None,
			Ordering::Equal => return Some(T::zero()),
			_ => {}
		}

		let maxshift: u32 = T::zero().leading_zeros() - 1;
		let shift: u32 = (maxshift - self.leading_zeros()) & !1;
		let mut bit = T::one().unsigned_shl(shift);
		let mut n = *self;
		let mut result = T::zero();
		while bit != T::zero()
		{
			if n >= (result + bit)
			{
				n = n - (result + bit);
				result = result.unsigned_shr(1) + bit;
			}
			else
			{
				result = result.unsigned_shr(1);
			}
			bit = bit.unsigned_shr(2);
		}
		Some(result)
	}
}
