use alloc::vec::Vec;
use crate::data::json::{JsonObj, JsonVal, NumVal};
//use crate::data::json::parser::{implementation::SimpleErr, lit, parse::{ Cat, Cat3,
use crate::lit;


lit!
{
	pub Whitespace_Char => '\u{0020}' | '\u{000D}' | '\u{000A}' | '\u{0009}';
}
