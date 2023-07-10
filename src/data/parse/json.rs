use alloc::vec::Vec;
use core::marker::PhantomData;

use crate::data::json::{JsonObj, JsonVal, NumVal};
use crate::data::json::parser::implementation::{SimpleErr, SimplePos};
use crate::data::json::parser::traits::{Err, Input, Pos, ResultOf};


#[derive(Clone)]
pub struct ParseOpts
{
	pub max_nest_level: Option<u32>,
}

impl Default for ParseOpts
{
	fn default() -> Self
	{
		ParseOpts
		{
			max_nest_level: Some(100),
		}
	}
}


#[derive(Clone)]
pub struct ParseContext
{
	nest_level: u32,
	opts: ParseOpts,
}


impl ParseContext
{
	pub fn new(opts: ParseOpts) -> Self
	{
		Self
		{
			nest_level: 0,
			opts,
		}
	}

	pub fn opts(&self) -> &ParseOpts
	{
		&self.opts
	}

	pub fn nest<I: Input>(&self, input: &I, pos: I::Pos) -> Result<Self, I::Err>
	{
		if Some(self.nest_level) == self.opts.max_nest_level
		{
			Err(input.err_at(pos, "[ERR] EXCEEDED NEST LEVEL"))
		}
		else
		{
			Ok(Self
			{
				nest_level: self.nest_level + 1,
				opts: self.opts.clone(),
			})
		}
	}
}


pub trait Parse<I: Input>
{
	type Output;

	fn parse(input: &I, current: I::Pos, context: &ParseContext) -> ResultOf<I, Self::Output>;
}


pub trait Predicate<T>
{
	fn evaluate(t: &T) -> bool;
}


pub struct ExpectCharacter<P>(PhantomData<P>);


impl<P: Predicate<char>, I: Input> Parse<I> for ExpectCharacter<P>
{
	type Output = char;

	fn parse(input: &I, current: I::Pos, _context: &ParseContext) -> ResultOf<I, Self::Output>
	{
		let (c, next) = input.next(current).map_err(|e| e.new_cause(current, "ExpectCharacter"))?;

		if P::evaluate(&c)
		{
			Ok((c, next))
		}
		else
		{
			Err(input.err_at(current, "ExpectCharacter"))
		}
	}
}

pub struct Null;


impl<I: Input> Parse<I> for Null
{
	type Output = ();

	fn parse(_input: &I, current: I::Pos, _context: &ParseContext) -> ResultOf<I, Self::Output>
	{
		Ok(((), current))
	}
}


pub struct Cat<P, P2>(PhantomData<(P, P2)>);


impl<I: Input, P: Parse<I>, P2: Parse<I>> Parse<I> for Cat<P, P2>
{
	type Output = (P::Output, P2::Output);

	fn parse(input: &I, current: I::Pos, context: &ParseContext) -> ResultOf<I, Self::Output>
	{
		let (output1, pos) = P::parse(input, current, context).map_err(|e| e.new_cause(current, "Cat1"))?;

		let (output2, pos) = P2::parse(input, pos, context).map_err(|e| e.new_cause(current, "Cat2"))?;

		Ok(((output1, output2), pos))
	}
}


pub type Cat3<P, P2, P3> = Cat<P, Cat<P2, P3>>;
pub type Cat4<P, P2, P3, P4> = Cat<P, Cat<P2, Cat<P3, P4>>>;
pub type Cat5<P, P2, P3, P4, P5> = Cat<P, Cat<P2, Cat<P3, Cat<P4, P5>>>>;


pub enum Either<A, B>
{
	A(A),
	B(B),
}


pub struct OneOf<P, P2>(PhantomData<(P, P2)>);


impl<I: Input, P:Parse<I>, P2: Parse<I>> Parse<I> for OneOf<P, P2>
{
	type Output = Either<P::Output, P2::Output>;

	fn parse(input: &I, current: I::Pos, context: &ParseContext) -> ResultOf<I, Self::Output>
	{
		P::parse(input, current, context).map(|(output, pos)| (Either::A(output), pos)).or_else(|_|
		{
			P2::parse(input, current, context).map(|(output, pos)| (Either::B(output), pos))
		})
		.map_err(|e| e.new_cause(current, "OneOf"))
	}
}


pub type OneOf3<P, P2, P3> = OneOf<P, OneOf<P2, P3>>;
pub type OneOf4<P, P2, P3, P4> = OneOf<P, OneOf3<P2, P3, P4>>;
pub type OneOf5<P, P2, P3, P4, P5> = OneOf<P, OneOf4<P2, P3, P4, P5>>;
pub type OneOf6<P, P2, P3, P4, P5, P6> = OneOf<P, OneOf5<P2, P3, P4, P5, P6>>;
pub type OneOf7<P, P2, P3, P4, P5, P6, P7> = OneOf<P, OneOf6<P2, P3, P4, P5, P6, P7>>;
pub type OneOf8<P, P2, P3, P4, P5, P6, P7, P8> = OneOf<P, OneOf7<P2, P3, P4, P5, P6, P7, P8>>;
pub type OneOf9<P, P2, P3, P4, P5, P6, P7, P8, P9> = OneOf<P, OneOf8<P2, P3, P4, P5, P6, P7, P8, P9>>;

pub type ZeroOrOne<P> = OneOf<P, Null>;
pub type ZeroOrMore<P> = OneOf<OneOrMore<P>, Null>;

pub struct OneOrMore<P>(PhantomData<P>);


impl<I: Input, P: Parse<I>> Parse<I> for OneOrMore<P>
{
	type Output = Vec<P::Output>;

	fn parse(input: &I, current: I::Pos, context: &ParseContext) -> ResultOf<I, Self::Output>
	{
		let mut output_list = Vec::new();
		let (output, mut pos) = P::parse(input, current, context).map_err(|e| e.new_cause(current, "OneOrMore"))?;
		output_list.push(output);

		loop
		{
			if let Ok((output, next_pos)) = P::parse(input, pos, context)
			{
				pos = next_pos;
				output_list.push(output);
			}
			else
			{
				return Ok((output_list, pos));
			}
		}
	}
}


impl Input for &str
{
	type Pos = SimplePos;
	type Err = SimpleErr;

	fn next(&self, pos: Self::Pos) -> Result<(char, Self::Pos), Self::Err>
	{
		self.chars()
			.nth(pos.idx() as usize)
			.ok_or_else(|| self.err_at(pos, "[ERR] OUT OF BOUNDS"))
			.map(|c| (c, pos.next(c)))
	}

	fn next_range(&self, start: Self::Pos, counts: u32) -> Result<(&str, Self::Pos), Self::Err>
	{
		let startidx = start.idx() as usize;
		let range = startidx..startidx + counts as usize;
		self.get(range)
			.map(|s|
			{
				let mut pos = start;
				for c in s.chars()
				{
					pos = pos.next(c);
				}

				(s, pos)
			})

			.ok_or_else(|| self.err_at(start, "[ERR] OUT OF BOUNDS"))
	}


	fn err_at(&self, pos: Self::Pos, cause: &'static str) -> Self::Err
	{
		let mut causes = Vec::new();
		causes.push((pos, cause));
		SimpleErr
		{
			causes
		}
	}


	fn end(&self, pos: Self::Pos) -> bool
	{
		pos.idx() as usize >= self.len()
	}
}



#[macro_export]
macro_rules! lit
{
	(
		$(
			$( #[ $attr:meta ] )*
			$vis:vis $name:ident => $($($value:literal)..=+)|+;
		)*
	) => {
		$(
			crate::lit!{
				IMPL
				$( #[ $attr ] )*
				$vis $name => $($($value)..=+)|+
			}
		)*
	};

	(
		IMPL
		$( #[ $attr:meta ] )*
		$vis:vis $name:ident => $($($value:literal)..=+)|+
	) => (
		$crate::paste::item!
		{
			$vis struct [< $name Predicate >];
			impl $crate::data::parse::json::Predicate<char> for [< $name Predicate >]
			{
				fn evaluate(c: &char) -> bool
				{
					match *c
					{
						$($($value)..=+)|+ => true,
						_ => false
					}
				}
			}

			$( #[ $attr ] )*
			$vis type $name = $crate::data::parse::json::ExpectCharacter<[< $name Predicate >]>;
		}
	);
}



#[macro_export]
macro_rules! parsers
{
	(
		$(
			$( #[ $attr:meta ] )*
			$vis:vis $name:ident = $type:ty, $output_type:ty, ($output:ident) => $body:block;
		)*
	) => {
		$(
			$vis struct $name;
			impl<I: $crate::data::json::parser::traits::Input> $crate::data::parse::json::Parse<I> for $name
			{
				type Output = $output_type;

				fn parse(input: &I, current: I::Pos, context: $ParseContext) -> $crate::data::json::parser::traits::ResultOf<I, Self::Output>
				{
					let ($output, pos) = <$type as $crate::data::parse::json::Parse<I>>::parse<input, current, context>
						.map_err(|e| <I::Err as $crate::data::json::traits::Err>::new_cause(e, current, stringify!($name)))?;
					let res = $body;
					Ok((res, pos))
				}	
			}
		)*
	};
}
