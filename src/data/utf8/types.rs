// src/data/utf8/types.rs
//
// UTF-8 types.
// Thank you to Alacritty for their crate "utf8parse"!



// What to do when byte is received
#[derive(Debug, Copy, Clone)]
pub enum Action
{
	InvalidSeq 	= 		0,
	EmitByte 	= 		1,
	SetByte1 	= 		2,
	SetByte2 	= 		3,
	SetByte2Top 	= 		4,
	SetByte3 	= 		5,
	SetByte3Top 	= 		6,
	SetByte4 	= 		7,
}


// These are the states that the parser can be in
#[allow(non_camel_case_types)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum State
{
	#[default]
	Ground 		=		0,
	Tail3 		=		1,
	Tail2 		=		2,
	Tail1 		= 		3,
	U3_2_e0 	= 		4,
	U3_2_ed 	= 		5,
	UTF8_4_3_f0 	= 		6,
	UTF8_4_3_f4 	= 		7,
}


impl State
{
	#[inline]
	pub fn adv(self, byte: u8) -> (State, Action)
	{
		match self
		{
			State::Ground => match byte
			{
				0x00..=0x7f => (State::Ground, Action::EmitByte),
				0xc2..=0xdf => (State::Tail1, Action::SetByte2Top),
				0xe0 => (State::U3_2_e0, Action::SetByte2Top),
				0xe1..=0xec => (State::Tail2, Action::SetByte3Top),
				0xed => (State::U3_2_ed, Action::SetByte3Top),
				0xee..=0xef => (State::Tail2, Action::SetByte3Top),
				0xf0 => (State::UTF8_4_3_f0, Action::SetByte4),
				0xf1..=0xf3 => (State::Tail3, Action::SetByte4),
				0xf4 => (State::UTF8_4_3_f4, Action::SetByte4),
				_ => (State::Ground, Action::InvalidSeq),
			},

			State::U3_2_e0 => match byte
			{
				0xa0..=0xbf => (State::Tail1, Action::SetByte2),
				_ => (State::Ground, Action::InvalidSeq),
			},

			State::U3_2_ed => match byte
			{
				0x80..=0x9f => (State::Tail1, Action::SetByte2),
				_ => (State::Ground, Action::InvalidSeq),
			},

			State::UTF8_4_3_f0 => match byte
			{
				0x90..=0xbf => (State::Tail2, Action::SetByte3),
				_ => (State::Ground, Action::InvalidSeq),
			},

			State::UTF8_4_3_f4 => match byte
			{
				0x80..=0x8f => (State::Tail2, Action::SetByte3),
				_ => (State::Ground, Action::InvalidSeq),
			},

			State::Tail3 => match byte
			{
				0x80..=0xbf => (State::Tail2, Action::SetByte3),
				_ => (State::Ground, Action::InvalidSeq),
			},


			State::Tail2 => match byte
			{
				0x80..=0xbf => (State::Tail1, Action::SetByte2),
				_ => (State::Ground, Action::InvalidSeq),
			},

			State::Tail1 => match byte
			{
				0x80..=0xbf => (State::Ground, Action::SetByte1),
				_ => (State::Ground, Action::InvalidSeq),
			},
		}
	}
}
