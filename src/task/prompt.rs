// src/task/prompt.rs
//
// Creates a prompt.

use alloc::{boxed::Box, string::{String, ToString}, vec::Vec};
use core::cmp;
use vte::{Params, Parser, Perform};


pub struct Prompt
{
	cursor: usize,

	// Uses UTF-32
	line: Vec<char>,

	// The offset of the prompt
	offset: usize,
}


impl Prompt
{
	pub fn new() -> Self
	{
		cursor: 0,
		line: Vec::with_capacity(80),
		offset: 0,
	}
	}

	pub fn input(&mut self, prompt: &str) -> Option<String>
	{
		print!("{}", prompt);
		self.cursor = self.offset;
		self.line = Vec::with_capacity(80);
		self.offset = offset_from_prompt(prompt);
