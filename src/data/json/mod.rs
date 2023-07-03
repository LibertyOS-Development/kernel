// src/data/json/mod.rs
//
// This is the mod.rs file for data::json, and this file establishes the
// kernel's ability to understand JSON. The other modules within this directory
// expand upon the code within this file.

pub mod parser;


pub mod parse;
pub mod traits;


use alloc::{ vec::Vec, string::ToString };
use crate::data::json::traits::Serialize;


#[derive(Clone, PartialEq, Copy)]
pub struct NumVal
{
	pub int: u64,
	pub frac: u64,
	pub frac_len: u32,
	pub exp: i32,
	pub neg: bool,
}


impl NumVal
{
	pub fn to_f64(self) -> f64
	{
		self.into()
	}
}


impl Into<f64> for NumVal
{
	fn into(self) -> f64
	{
		use num_traits::float::FloatCore as _;

		let sign = if self.neg
		{
			-1.0
		}
		else
		{
			1.0
		};

		(self.int as f64 + self.frac as f64 / 10f64.powi(self.frac_len as i32))
			* 10f64.powi(self.exp)
			* sign
	}
}

pub type JsonObj = Vec<(Vec<char>, JsonVal)>;


#[derive(Clone, PartialEq)]
pub enum JsonVal
{
	Obj(JsonObj),
	Array(Vec<JsonVal>),
	Str(Vec<char>),
	Num(NumVal),
	Boolean(bool),
	Null,
}


impl JsonVal
{
	// Object
	pub fn is_obj(&self) -> bool
	{
		match self
		{
			JsonVal::Obj(_) => true,
			_ => false,
		}
	}

	pub fn as_obj(&self) -> Option<&[(Vec<char>, JsonVal)]>
	{
		match self
		{
			JsonVal::Obj(obj) => Some(obj),
			_ => None,
		}
	}

	pub fn to_obj(self) -> Option<JsonObj>
	{
		match self
		{
			JsonVal::Obj(obj) => Some(obj),
			_ => None,
		}
	}


	// Array
	pub fn is_array(&self) -> bool
	{
		match self
		{
			JsonVal::Array(_) => true,
			_ => false,
		}
	}

	pub fn as_array(&self) -> Option<&[JsonVal]>
	{
		match self
		{
			JsonVal::Array(arr) => Some(arr),
			_ => None,
		}
	}

	pub fn to_array(self) -> Option<Vec<JsonVal>>
	{
		match self
		{
			JsonVal::Array(arr) => Some(arr),
			_ => None,
		}
	}


	// String
	pub fn is_str(&self) -> bool
	{
		match self
		{
			JsonVal::Str(_) => true,
			_ => false,
		}
	}

	pub fn as_str(&self) -> Option<&[char]>
	{
		match self
		{
			JsonVal::Str(s) => Some(s),
			_ => None,
		}
	}

	pub fn to_str(self) -> Option<Vec<char>>
	{
		match self
		{
			JsonVal::Str(s) => Some(s),
			_ => None,
		}
	}


	// Number
	pub fn is_num(&self) -> bool
	{
		match self
		{
			JsonVal::Num(_) => true,
			_ => false,
		}
	}

	pub fn as_num(&self) -> Option<&NumVal>
	{
		match self
		{
			JsonVal::Num(n) => Some(n),
			_ => None,
		}
	}

	pub fn to_num(self) -> Option<NumVal>
	{
		match self
		{
			JsonVal::Num(n) => Some(n),
			_ => None,
		}
	}


	// Bool
	pub fn is_bool(&self) -> bool
	{
		match self
		{
			JsonVal::Boolean(_) => true,
			_ => false,
		}
	}

	pub fn as_bool(&self) -> Option<&bool>
	{
		match self
		{
			JsonVal::Boolean(b) => Some(b),
			_ => None,
		}
	}

	pub fn to_bool(self) -> Option<bool>
	{
		match self
		{
			JsonVal::Boolean(b) => Some(b),
			_ => None,
		}
	}


	// Null
	pub fn is_null(&self) -> bool
	{
		match self
		{
			JsonVal::Null => true,
			_ => false,
		}
	}
}


impl Serialize for NumVal
{
	fn serialize_to(&self, buffer: &mut Vec<u8>, _indent: u32, _level: u32)
	{
		if self.neg
		{
			buffer.push(b'-');
		}

		buffer.extend_from_slice(self.int.to_string().as_bytes());

		if self.frac > 0
		{
			buffer.push(b'.');

			let frac_num = self.frac.to_string();
			let frac_len = self.frac_len as usize;

			for _ in 0..frac_len - frac_num.len()
			{
				buffer.push(b'0');
			}

			buffer.extend_from_slice(frac_num.as_bytes())
		}

		if self.exp != 0
		{
			buffer.push(b'e');

			if self.exp < 0
			{
				buffer.push(b'-');
			}

			buffer.extend_from_slice(self.exp.abs().to_string().as_bytes());
		}
	}
}


fn push_str(buffer: &mut Vec<u8>, chars: &Vec<char>)
{
	buffer.push('"' as u8);

	for ch in chars
	{
		match ch
		{
			'\x08' => buffer.extend_from_slice(br#"\b"#),
			'\x0c' => buffer.extend_from_slice(br#"\f"#),
			'\n' => buffer.extend_from_slice(br#"\n"#),
			'\r' => buffer.extend_from_slice(br#"\r"#),
			'\t' => buffer.extend_from_slice(br#"\t"#),
			'\"' => buffer.extend_from_slice(br#"\""#),
			'\\' => buffer.extend_from_slice(br#"\\"#),

			_ => match ch.len_utf8()
			{
				1 =>
				{
					let mut buff = [0u8; 1];
					ch.encode_utf8(&mut buff);
					buffer.push(buff[0]);
				}

				2 =>
				{
					let mut buff = [0u8; 2];
					ch.encode_utf8(&mut buff);
					buffer.extend_from_slice(&buff);
				}

				3 =>
				{
					let mut buff = [0u8; 3];
					ch.encode_utf8(&mut buff);
					buffer.extend_from_slice(&buff);
				}

				4 =>
				{
					let mut buff = [0u8; 4];
					ch.encode_utf8(&mut buff);
					buffer.extend_from_slice(&buff);
				}

				_ => panic!("[ERR] INVALID UTF-8 CHARACTER"),
			},
		}
	}

	buffer.push('"' as u8);
}


fn push_new_ln_indent(buffer: &mut Vec<u8>, indent: u32, level: u32)
{
	if indent > 0
	{
		buffer.push('\n' as u8);
	}

	let count = (indent * level) as usize;
	buffer.reserve(count);

	for _ in 0..count
	{
		buffer.push(' ' as u8);
	}
}


impl Serialize for JsonVal
{
	fn serialize_to(&self, buffer: &mut Vec<u8>, indent: u32, level: u32)
	{
		match self
		{
			JsonVal::Obj(obj) =>
			{
				buffer.push('{' as u8);

				if obj.len() > 0
				{
					push_new_ln_indent(buffer, indent, level + 1);
					push_str(buffer, &obj[0].0);
					buffer.push(':' as u8);

					if indent > 0
					{
						buffer.push(' ' as u8);
					}

					obj[0].1.serialize_to(buffer, indent, level + 1);

					for (key, val) in obj.iter().skip(1)
					{
						buffer.push(',' as u8);
						push_new_ln_indent(buffer, indent, level + 1);
						push_str(buffer, key);
						buffer.push(':' as u8);

						if indent > 0
						{
							buffer.push(' ' as u8);
						}

						val.serialize_to(buffer, indent, level + 1);
					}

					push_new_ln_indent(buffer, indent, level);
					buffer.push('}' as u8);
				}
				else
				{
					buffer.push('}' as u8);
				}
			}

			JsonVal::Array(arr) =>
			{
				buffer.push('[' as u8);

				if arr.len() > 0
				{
					push_new_ln_indent(buffer, indent, level + 1);
					arr[0].serialize_to(buffer, indent, level + 1);

					for val in arr.iter().skip(1)
					{
						buffer.push(',' as u8);
						push_new_ln_indent(buffer, indent, level + 1);
						val.serialize_to(buffer, indent, level);
					}

					push_new_ln_indent(buffer, indent, level);
					buffer.push(']' as u8);
				}
				else
				{
					buffer.push(']' as u8);
				}
			}

			JsonVal::Str(str) => push_str(buffer, str),
			JsonVal::Num(num) => num.serialize_to(buffer, indent, level),
			JsonVal::Boolean(true) => buffer.extend_from_slice(b"true"),
			JsonVal::Boolean(false) => buffer.extend_from_slice(b"false"),
			JsonVal::Null => buffer.extend_from_slice(b"null"),
		}
	}
}
