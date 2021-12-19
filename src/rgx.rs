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
			'w' => MetaCharacter::Alphanum,
			'D' => MetaCharacter::NonWhitespace,
			'W' => MetaCharacter::NonAlphanum,
			_ => MetaCharacter::Lit(c),
		}
	}
	fn contains(&self, c: char) -> bool
	{
		match self
		{
			MetaCharacter::Any => true,
			MetaCharacter::Num => c.is_numeric(),
			MetaCharacter::Whitespace => c.is_whitespace(),
			MetaCharacter::Alphanum => c.is_alphanumeric(),
			MetaCharacter::NonNum => !c.is_numeric(),
			MetaCharacter::NonWhitespace => !c.is_whitespace(),
			MetaCharacter::NonAlphanum => !c.is_alphanumeric(),
			MetaCharacter::Lit(lc) => c == *lc,
		}
	}
}


#[derive(Debug)]
pub struct Rgx(String);

impl Rgx
{
	pub fn new(re: &str) -> Self
	{
		Self(re.to_string())
	}
	pub fn ismatch(&self, text: &str) -> bool
	{
		self.find(text).is_some()
	}
	pub fn find(&self, text: &str) -> Option<(usize, usize)>
	{
		let text: Vec<char> = text.chars().collect();
		let re: Vec<char> = self.0.chars().collect();
		let mut start = 0;
		let mut end = 0;
		if ismatch(&re[..], &text[..], &mut start, &mut end)
		{
			Some((start, end))
		}
		else
		{
			None
		}
	}
}


fn ismatch(re: &[char], text: &[char], start: &mut usize, end: &mut usize) -> bool
{
	if re.len() == 0
	{
		return true;
	}
	if re[0] == '^'
	{
		*end = 1;
		return ismatch_here(&re[1..], text, end);
	}
	let mut i = 0;
	let n = text.len();
	loop
	{
		*start = i;
		*end = i;
		if ismatch_here(re, &text[i..], end)
		{
			return true;
		}
		if i == n
		{
			return false;
		}
		i += 1;
	}
}

fn ismatch_here(re: &[char], text: &[char], end: &mut usize) -> bool
{
	if re.len() == 0
	{
		return true;
	}
	if re[0] == '$'
	{
		return text.len() == 0;
	}
	let (mc, i) = if re.len() > 1 && re[0] == '\\'
	{
		(MetaCharacter::fromesc(re[1]), 1)
	}
	else
	{
		(MetaCharacter::from(re[0]), 0)
	};
	if re.len() > i + 1
	{
		let lazy = re.len() > i + 2 && re[i + 2] == '?';
		let j = if lazy
		{
			i + 3
		}
		else
		{
			i + 2
		};
		match re[i + 1]
		{
			'*' => return ismatch_star(lazy, mc, &re[j..], text, end),
			'+' => return ismatch_plus(lazy, mc, &re[j..], text, end),
			'?' => return ismatch_qst(lazy, mc, &re[j..], text, end),
			_ => {}
		}
	}
	if text.len() != 0 && mc.contains(text[0])
	{
		*end += 1;
		let j = i + 1;
		return ismatch_here(&re[j..], &text[1..], end);
	}
	false
}

fn ismatch_star(lazy: bool, mc: MetaCharacter, re: &[char], text: &[char], end: &mut usize) -> bool
{
	ismatch_char(lazy, mc, re, text, .., end)
}

fn ismatch_plus(lazy: bool, mc: MetaCharacter, re: &[char], text: &[char], end: &mut usize) -> bool
{
	ismatch_char(lazy, mc, re, text, 1.., end)
}

fn ismatch_qst(lazy: bool, mc: MetaCharacter, re: &[char], text: &[char], end: &mut usize) -> bool
{
	ismatch_char(lazy, mc, re, text, ..2, end)
}

fn ismatch_char<T: RangeBounds<usize>>(lazy: bool, mc: MetaCharacter, re: &[char], text: &[char], range: T, end: &mut usize) -> bool
{
	let mut i = 0;
	let n = text.len();
	if !lazy
	{
		loop
		{
			if i == n || !(mc.contains(text[i]))
			{
				break;
			}
			i += 1;
		}
	}
	loop
	{
		if ismatch_here(re, &text[i..], end) && range.contains(&i)
		{
			*end += i;
			return true;
		}
		if lazy
		{
			if i == n || !(mc.contains(text[i]))
			{
				return false;
			}
			i += i;
		}
		else
		{
			if i == 0
			{
				return false;
			}
			i -= 1;
		}
	}
}
