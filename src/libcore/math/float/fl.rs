// src/math/float/fl.rs
//
// This is the module that establishes the basic functionality of a float.

#![allow(unconditional_recursion)]

use core::{ cmp::Ordering, fmt::{self, Display, LowerExp, UpperExp}, iter::{Product, Sum}, num::ParseFloatError, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign}, str::FromStr};
use num_traits::{Inv, Num, One, Zero};

// Sign mask:
pub const SIGN_M: u32 = 0b1000_0000_0000_0000_0000_0000_0000_0000;

// Exponent mask:
pub const EXP_M: u32 = 0b0111_1111_1000_0000_0000_0000_0000_0000;

// Mantissa mask:
pub const MANT_M: u32 = 0b0000_0000_0111_1111_1111_1111_1111_1111;

// Exponent bias:
pub const EXP_B: u32 = 127;

// Mantissa bits:
pub const MANT_B: u32 = 23;


// This is a wrapper for 32-bit floating-points.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct FL32(pub f32);

impl FL32
{
	// The value of zero (0.0)
	pub const ZERO: Self = Self(0.0);

	// The value of one (1.0)
	pub const ONE: Self = Self(1.0);

	// The base of the internal-representation of f32
	pub const FBASE: u32 = f32::RADIX;

	// Count of significant digits, using base-2.
	pub const MANT_DIG: u32 = f32::MANTISSA_DIGITS;

	// Approximate value of digits, using base-10.
	pub const DIG: u32 = f32::DIGITS;

	// The machine-epsilon value of f32
	pub const EPSILON: Self = Self(f32::EPSILON);

	// The smallest value of f32
	pub const MINIM: Self = Self(f32::MIN);

	// The smallest positive value of f32
	pub const MINIM_POS: Self = Self(f32::MIN_POSITIVE);

	// The greatest value of f32
	pub const MAXIM: Self = Self(f32::MAX);

	// The minimum value of 2^x, plus one (+1)
	pub const MINIM_EXP_2: i32 = f32::MIN_EXP;

	// The greatest value of 2^x
	pub const MAXIM_EXP_2: i32 = f32::MAX_EXP;

	// The minimum value of 10^x
	pub const MINIM_EXP_10: i32 = f32::MIN_10_EXP;

	// The greatest value of 10^x
	pub const MAXIM_EXP_10: i32 = f32::MAX_10_EXP;

	// "NOT A NUMBER"
	pub const NAN: Self = Self(f32::INFINITY);

	// Negative infinity
	pub const NEGINF: Self = Self(f32::NEG_INFINITY);

	// This will return "true" if the value is NAN:
	#[inline]
	pub fn nan(self) -> bool
	{
		self.0.is_nan()
	}

	// This will return "true" if the value is negative/positive infinity, or "false" if the value is anything else:
	#[inline]
	pub fn infinite(self) -> bool
	{
		self.0.is_infinite()
	}

	// This will return "true" if the value is not negative/positive infinity, nor is the value "not-a-number":
	#[inline]
	pub fn finite(self) -> bool
	{
		self.0.is_finite()
	}

	// This will return "true" if the value is positive:
	// NOTE: "+0.0", positive NAN, and positive infinity will all return "true".
	#[inline]
	pub fn positive(self) -> bool
	{
		self.0.is_sign_positive()
	}

	// This will return "true" if the value is negative:
	// NOTE: "-0.0", negative NAN, and negative infinity will all return "true".
	#[inline]
	pub fn negative(self) -> bool
	{
		self.0.is_sign_negative()
	}

	// This is a raw-tranmutation of f32, to u32:
	#[inline]
	pub fn conv_to_bits(self) -> u32
	{
		self.0.to_bits()
	}

	// This is a raw-transmutation of u32, to f32:
	#[inline]
	pub fn conv_from_bits(v: u32) -> Self
	{
		Self(f32::from_bits(v))
	}

	// This extracts exponent bits:
	pub fn ext_exp_bits(self) -> u32
	{
		(self.conv_to_bits() & EXP_M)
			.overflowing_shr(MANT_B)
			.0
	}

	// This extracts the exponent of an float value:
	pub fn ext_exp_val(self) -> i32
	{
		(self.ext_exp_bits() as i32) - EXP_B as i32
	}

	// This removes negative/positive (-/+) signs:
	pub fn wosign(self) -> Self
	{
		Self::conv_from_bits(self.conv_to_bits() & !SIGN_M)
	}

	// This sets the exponent to a specific value:
	pub fn setexp(self, exp: i32) -> Self
	{
		debug_assert!(exp <= 127 && exp >= -128);
		let woexp: u32 = self.conv_to_bits() & !EXP_M;
		let onlyexp: u32 = ((exp + EXP_B as i32) as u32)
			.overflowing_shl(MANT_B)
			.0;
		Self::conv_from_bits(woexp | onlyexp)
	}

	// This checks if the value is an integer:
	pub fn integer(&self) -> bool
	{
		let exp = self.ext_exp_val();
		let selfbits = self.conv_to_bits();

		// This will prevent an opposite shift, in the event of a negative exponent:
		let exp_clamped = i32::max(exp, 0) as u32;

		// This finds the portion of the fraction that would be left behind:
		let fract_part = (selfbits).overflowing_shl(exp_clamped).0 & MANT_M;

		// If fract_part has any value, the value is not an integer.
		fract_part == 0
	}

	// This checks if the value of the floating-point is even:
	fn even(&self) -> bool
	{
		if self.ext_exp_val() >= 31
		{
			true
		}
		else
		{
			(self.0 as i32) % 2 == 0
		}
	}
}


/*
	IMPLEMENTATIONS
*/

impl Add for FL32
{
	type Output = FL32;
	#[inline]
	fn add(self, rhs: FL32) -> FL32
	{
		FL32(self.0 + rhs.0)
	}
}

impl Add<f32> for FL32
{
	type Output = FL32;
	#[inline]
	fn add(self, rhs: f32) -> FL32
	{
		FL32(self.0 + rhs)
	}
}

impl Add<FL32> for f32
{
	type Output = FL32;
	#[inline]
	fn add(self, rhs: FL32) -> FL32
	{
		FL32(self + rhs.0)
	}
}

impl AddAssign for FL32
{
	#[inline]
	fn add_assign(&mut self, rhs: FL32)
	{
		self.0 += rhs.0;
	}
}

impl AddAssign<f32> for FL32
{
	#[inline]
	fn add_assign(&mut self, rhs: f32)
	{
		self.0 += rhs;
	}
}

impl AddAssign<FL32> for f32
{
	#[inline]
	fn add_assign(&mut self, rhs: FL32)
	{
		*self += rhs.0;
	}
}

impl Display for FL32
{
	#[inline]
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(fmt, "{}", self.0)
	}
}

impl Div for FL32
{
	type Output = FL32;
	#[inline]
	fn div(self, rhs: FL32) -> FL32
	{
		FL32(self.0 / rhs.0)
	}
}

impl Div<f32> for FL32
{
	type Output = FL32;
	#[inline]
	fn div(self, rhs: f32) -> FL32
	{
		FL32(self.0 / rhs)
	}
}

impl Div<FL32> for f32
{
	type Output = FL32;
	#[inline]
	fn div(self, rhs: FL32) -> FL32
	{
		FL32(self / rhs.0)
	}
}

impl DivAssign for FL32
{
	#[inline]
	fn div_assign(&mut self, rhs: FL32)
	{
		self.0 /= rhs.0;
	}
}

impl DivAssign<f32> for FL32
{
	#[inline]
	fn div_assign(&mut self, rhs: f32)
	{
		self.0 /= rhs;
	}
}

impl DivAssign<FL32> for f32
{
	#[inline]
	fn div_assign(&mut self, rhs: FL32)
	{
		*self /= rhs.0;
	}
}

impl From<f32> for FL32
{
	#[inline]
	fn from(n: f32) -> FL32
	{
		FL32(n)
	}
}

impl From<FL32> for f32
{
	#[inline]
	fn from(n: FL32) -> f32
	{
		n.0
	}
}

impl From<i8> for FL32
{
	#[inline]
	fn from(n: i8) -> FL32
	{
		FL32(n.into())
	}
}

impl From<i16> for FL32
{
	#[inline]
	fn from(n: i16) -> FL32
	{
		FL32(n.into())
	}
}

impl From<u8> for FL32
{
	#[inline]
	fn from(n: u8) -> FL32
	{
		FL32(n.into())
	}
}

impl From<u16> for FL32
{
	#[inline]
	fn from(n: u16) -> FL32
	{
		FL32(n.into())
	}
}

impl FromStr for FL32
{
	type Err = ParseFloatError;
	#[inline]
	fn from_str(src: &str) -> Result<FL32, ParseFloatError>
	{
		f32::from_str(src).map(FL32)
	}
}

impl Inv for FL32
{
	type Output = Self;
	fn inv(self) -> Self
	{
		self.inv()
	}
}

impl LowerExp for FL32
{
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(f, "{:e}", self.0)
	}
}

impl Mul for FL32
{
	type Output = FL32;
	#[inline]
	fn mul(self, rhs: FL32) -> FL32
	{
		FL32(self.0 * rhs.0)
	}
}

impl Mul<f32> for FL32
{
	type Output = FL32;
	#[inline]
	fn mul(self, rhs: f32) -> FL32
	{
		FL32(self.0 * rhs)
	}
}

impl Mul<FL32> for f32
{
	type Output = FL32;
	#[inline]
	fn mul(self, rhs: FL32) -> FL32
	{
		FL32(self * rhs.0)
	}
}

impl MulAssign for FL32
{
	#[inline]
	fn mul_assign(&mut self, rhs: FL32)
	{
		self.0 *= rhs.0;
	}
}

impl MulAssign<f32> for FL32
{
	#[inline]
	fn mul_assign(&mut self, rhs: f32)
	{
		self.0 *= rhs;
	}
}

impl MulAssign<FL32> for f32
{
	#[inline]
	fn mul_assign(&mut self, rhs: FL32)
	{
		*self *= rhs.0;
	}
}

impl Neg for FL32
{
	type Output = FL32;
	#[inline]
	fn neg(self) -> FL32
	{
		FL32(-self.0)
	}
}

impl Num for FL32
{
	type FromStrRadixErr = num_traits::ParseFloatError;
	fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr>
	{
		f32::from_str_radix(str, radix).map(Self)
	}
}

impl One for FL32
{
	fn one() -> Self
	{
		Self::ONE
	}
	fn is_one(&self) -> bool
	{
		Self::ONE == *self
	}
}

impl PartialEq<f32> for FL32
{
	fn eq(&self, other: &f32) -> bool
	{
		self.0.eq(other)
	}
}

impl PartialEq<FL32> for f32
{
	fn eq(&self, other: &FL32) -> bool
	{
		self.eq(&other.0)
	}
}

impl PartialOrd<f32> for FL32
{
	fn partial_cmp(&self, other: &f32) -> Option<Ordering>
	{
		self.0.partial_cmp(other)
	}
}

impl PartialOrd<FL32> for f32
{
	fn partial_cmp(&self, other: &FL32) -> Option<Ordering>
	{
		self.partial_cmp(&other.0)
	}
}

impl Product for FL32
{
	#[inline]
	fn product<I>(iter: I) -> Self
	where
		I: Iterator<Item = FL32>,
	{
		FL32(f32::product(iter.map(f32::from)))
	}
}

impl Rem for FL32
{
	type Output = FL32;
	#[inline]
	fn rem(self, rhs: FL32) -> FL32
	{
		FL32(self.0 % rhs.0)
	}
}

impl Rem<f32> for FL32
{
	type Output = FL32;
	#[inline]
	fn rem(self, rhs: f32) -> FL32
	{
		FL32(self.0 % rhs)
	}
}

impl Rem<FL32> for f32
{
	type Output = FL32;
	#[inline]
	fn rem(self, rhs: FL32) -> FL32
	{
		FL32(self % rhs.0)
	}
}

impl RemAssign for FL32
{
	#[inline]
	fn rem_assign(&mut self, rhs: FL32)
	{
		self.0 %= rhs.0;
	}
}

impl RemAssign<f32> for FL32
{
	#[inline]
	fn rem_assign(&mut self, rhs: f32)
	{
		self.0 %= rhs;
	}
}

impl Sub for FL32
{
	type Output = FL32;
	#[inline]
	fn sub(self, rhs: FL32) -> FL32
	{
		FL32(self.0 - rhs.0)
	}
}

impl Sub<f32> for FL32
{
	type Output = FL32;
	#[inline]
	fn sub(self, rhs: f32) -> FL32
	{
		FL32(self.0 - rhs)
	}
}

impl Sub<FL32> for f32
{
	type Output = FL32;
	#[inline]
	fn sub(self, rhs: FL32) -> FL32
	{
		FL32(self - rhs.0)
	}
}

impl SubAssign for FL32
{
	#[inline]
	fn sub_assign(&mut self, rhs: FL32)
	{
		self.0 -= rhs.0;
	}
}

impl SubAssign<f32> for FL32
{
	#[inline]
	fn sub_assign(&mut self, rhs: f32)
	{
		self.0 -= rhs;
	}
}

impl SubAssign<FL32> for f32
{
	#[inline]
	fn sub_assign(&mut self, rhs: FL32)
	{
		*self -= rhs.0;
	}
}

impl Sum for FL32
{
	#[inline]
	fn sum<I>(iter: I) -> Self
	where
		I: Iterator<Item = FL32>,
	{
		FL32(f32::sum(iter.map(f32::from)))
	}
}

impl UpperExp for FL32
{
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(f, "{:E}", self.0)
	}
}

impl Zero for FL32
{
	fn zero() -> Self
	{
		Self::ZERO
	}
	fn is_zero(&self) -> bool
	{
		Self::ZERO == *self
	}
}
