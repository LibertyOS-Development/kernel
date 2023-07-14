// src/data/parse/utf8.rs
//
// Parser for UTF-8 data.
// Thank you to Alacritty!


use core::char;
use crate::data::utf8::types::{Action, State};


pub trait Receiver
{
	fn codepoint(&mut self, _: char);
	fn invalidseq(&mut self);
}


#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct Parse
{
	point: u32,
	state: State,
}


const CONTINUATION_MASK: u8 = 0b0011_1111;


impl Parse
{
	// Create a parser
	pub fn new() -> Parse
	{
		Parse
		{
			point: 0,
			// Set state to the default value
			state: State::Ground,
		}
	}

	// Advance the parser
	pub fn adv<R>(&mut self, receiver: &mut R, byte: u8)
	where
		R: Receiver,
	{
		let (state, action) = self.state.adv(byte);
		self.perform_action(receiver, byte, action);
		self.state = state;
	}

	fn perform_action<R>(&mut self, receiver: &mut R, byte: u8, action: Action)
	where
		R: Receiver,
	{
		match action
		{
			Action::InvalidSeq =>
			{
				self.point = 0;
				receiver.invalidseq();
			},

			Action::EmitByte =>
			{
				receiver.codepoint(byte as char);
			},

			Action::SetByte1 =>
			{
				let point = self.point | ((byte & CONTINUATION_MASK) as u32);
				let c = unsafe
				{
					char::from_u32_unchecked(point)
				};

				self.point = 0;

				receiver.codepoint(c);
			},

			Action::SetByte2 =>
			{
				self.point |= ((byte & CONTINUATION_MASK) as u32) << 6;
			},

			Action::SetByte2Top =>
			{
				self.point |= ((byte & 0b0001_1111) as u32) << 6;
			},

			Action::SetByte3 =>
			{
				self.point |= ((byte & CONTINUATION_MASK) as u32) << 12;
			},

			Action::SetByte3Top =>
			{
				self.point |= ((byte & 0b0000_1111) as u32) << 12;
			},

			Action::SetByte4 =>
			{
				self.point |= ((byte & 0b0000_0111) as u32) << 18;
			},
		}
	}
}

