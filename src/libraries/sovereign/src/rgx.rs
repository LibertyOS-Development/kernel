// rgx.rs

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::convert::From;
use core::ops::RangeBounds;


#[derive(Debug)]
enum MetaCharacter
{
	Any,
	Num,
	Whitespace,
	Alphanum,
	NonNum,
	NonWhitespace,
	NonAlphanum,
	Lit(char),
}

impl From<char> for MetaCharacter
{
	fn from(c: char) -> Self
	{
		match c
		{
			'.' => MetaCharacter::Any,
			_ => MetaCharacter::Lit(c),
		}
	}
}

trait MetaCharacterExt
{
	fn fromesc(c: char) -> Self;
	fn contains(&self, c: char) -> bool;
}

impl MetaCharacterExt for MetaCharacter
{
	fn fromesc(c: char) -> Self
	{
		match c
		{
			'd' => MetaCharacter::Num,
			's' => MetaCharacter::Whitespace,
