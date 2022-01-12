// src/libcore/sys/prompt.rs
//
// Handles creating a prompt.

/*
	IMPORTS
*/

use alloc::{boxed::Box, string::{String, ToString}, vec::Vec};
use core::cmp;
use vte::{Params, Parser, Perform};

use crate::{print, println, libcore::{fs}};


// Completion struct
pub struct Completion
{
	// Completer
	completer: Box<dyn Fn(&str) -> Vec<String>>,

	// Items
	items: Vec<String>,

	// Position
	pos: Option<usize>,
}


// History struct
pub struct History
{
	// Items
	items: Vec<String>,

	// Limit
	limit: usize,

	// Position
	pos: Option<usize>,
}


// Offset struct
struct Offset(usize);


// Prompt struct
pub struct Prompt
{
	// Completion
	pub compl: Completion,

	// History
	pub hist: History,

	// Offset the line according to the length of prompt string
	offset: usize,

	// Cursor
	cur: usize,

	// Uses UTF-32
	ln: Vec<char>,
}


// Implementation of the Completion struct
impl Completion
{
	// New
	pub fn new() -> Self
	{
		Self
		{
			// Completer
			completer: Box::new(nullcompleter),

			// Items
			items: Vec::new(),

			// Position
			pos: None,
		}
	}


	// Set
	pub fn set(&mut self, completer: &'static dyn Fn(&str) -> Vec<String>)
	{
		self.completer = Box::new(completer);
	}
}


// Implementation of the History struct
impl History
{
	// Add
	pub fn add(&mut self, item: &str)
	{
		let mut i = 0;

		while i < self.items.len()
		{
			if self.items[i] == item
			{
				self.items.remove(i);
			}
			else
			{
				i += 1;
			}
		}

		self.items.push(item.to_string());

		// When the number of items reaches the limit, remove the oldest items
		while self.items.len() > self.limit
		{
			self.items.remove(0);
		}
	}


	// Load
	pub fn load(&mut self, path: &str)
	{
		if let Ok(ln) = fs::read_to_str(path)
		{
			self.items = ln.split('\n').map(|s| s.to_string()).collect();
		}
	}


	// New
	pub fn new() -> Self
	{
		Self
		{
			items: Vec::new(),
			limit: 1000,
			pos: None,
		}
	}


	// Save
	pub fn save(&mut self, path: &str)
	{
		crate::libcore::fs::write(path, self.items.join("\n").as_bytes()).ok();
	}
}


// Implementation of the Perform trait for the Offset struct
impl Perform for Offset
{
	// Print
	fn print(&mut self, c: char)
	{
		self.0 += c.len_utf8();
	}
}


// Implementation of the Perform trait for the Prompt struct
impl Perform for Prompt
{
	// CSI dispatcher
	fn csi_dispatch(&mut self, params: &Params, _inter: &[u8], _ignore: bool, c: char)
	{
		match c
		{
			// Up
			'A' => self.handle_up(),

			// Down
			'B' => self.handle_down(),

			// Forward
			'C' => self.handle_forward(),

			// Backward
			'D' => self.handle_backward(),

			'~' =>
			{
				for param in params.iter()
				{
					// Delete
					if param[0] == 3
					{
						self.handle_del();
					}
				}
			},

			_ => {},
		}
	}


	// Execute
	fn execute(&mut self, b: u8)
	{
		let c = b as char;
		match c
		{
			// Backspace
			'\x08' => self.handle_backspace(),

			// Tab
			'\t' => self.handle_tab(),

			_ => {},
		}
	}


	// Print
	fn print(&mut self, c: char)
	{
		match c
		{
			// Delete
			'\x7f' => self.handle_del(),

			// Printable keys
			c => self.handle_printable(c),
		}
	}
}


// Implementation of the Prompt struct
impl Prompt
{
	// Update completion
	fn compupdate(&mut self)
	{
		if let Some(i) = self.compl.pos
		{
			let complete = self.compl.items[i].chars();
			self.cur += complete.clone().count();
			self.ln.extend(complete);
			self.compl.pos = None;
			self.compl.items = Vec::new();
		}
	}


	// Handle backspace key
	fn handle_backspace(&mut self)
	{
		self.compupdate();
		self.histupdate();
		if self.cur > self.offset
		{
			let i = self.cur - self.offset - 1;
			self.ln.remove(i);

			// Use UTF-32
			let s = &self.ln[i..];
			let n = s.len() + 1;

			// Use UTF-8
			let s: String = s.iter().collect();
			print!("\x08{} \x1b[{}D", s, n);
			self.cur -= 1;
		}
	}


	// Handle backward key
	fn handle_backward(&mut self)
	{
		self.compupdate();
		self.histupdate();
		if self.cur > self.offset
		{
			print!("\x1b[1D");
			self.cur -= 1;
		}
	}


	// Handle delete key
	fn handle_del(&mut self)
	{
		self.compupdate();
		self.histupdate();

		if self.cur < self.offset + self.ln.len()
		{
			let i = self.cur - self.offset;
			self.ln.remove(i);

			// Use UTF-32
			let s = &self.ln[i..];
			let n = s.len() + 1;

			// Use UTF-8
			let s: String = s.iter().collect();
			print!("{} \x1b[{}D", s, n);
		}
	}


	// Handle down key
	fn handle_down(&mut self)
	{
		self.compupdate();
		let n = self.hist.items.len();
		if n == 0
		{
			return;
		}

		let (bs, i) = match self.hist.pos
		{
			Some(i) => (self.hist.items[i].chars().count(), i + 1),
			None => return,
		};

		let (pos, ln) = if i < n
		{
			(Some(i), self.hist.items[i].clone())
		}
		else
		{
			(None, self.ln.iter().collect())
		};

		let erase = '\x08'.to_string().repeat(bs);
		print!("{}{}", erase, ln);
		self.cur = self.offset + ln.chars().count();
		self.hist.pos = pos;
	}


	// Handle forward key
	fn handle_forward(&mut self)
	{
		self.compupdate();
		self.histupdate();
		if self.cur < self.offset + self.ln.len()
		{
			print!("\x1b[1C");
			self.cur += 1;
		}
	}


	// Handle printable keys
	fn handle_printable(&mut self, c: char)
	{
		self.compupdate();
		self.histupdate();
		if crate::libcore::sys::console::canprint(c)
		{
			let i = self.cur - self.offset;
			self.ln.insert(i, c);

			// Use UTF-32
			let s = &self.ln[i..];
			let n = s.len();

			// Use UTF-8
			let s: String = s.iter().collect();
			print!("{} \x1b[{}D", s, n);
			self.cur += 1;
		}
	}


	// Handle tab key
	fn handle_tab(&mut self)
	{
		self.histupdate();
		let (bs, pos) = match self.compl.pos
		{
			Some(pos) =>
			{
				let n = self.compl.items.len();
				if n == 1
				{
					self.compupdate();
					return;
				}

				let bs = self.compl.items[pos].chars().count();
				if pos + 1 < n
				{
					(bs, pos + 1)
				}
				else
				{
					(bs, 0)
				}
			},

			None =>
			{
				let ln: String = self.ln.iter().collect();
				self.compl.items = (self.compl.completer)(&ln);
				if !self.compl.items.is_empty()
				{
					(0, 0)
				}
				else
				{
					return
				}
			},
		};

		let erase = "\x08".repeat(bs);
		let complete = &self.compl.items[pos];
		print!("{}{}", erase, complete);
		self.compl.pos = Some(pos);
	}


	// Handle up key
	fn handle_up(&mut self)
	{
		self.compupdate();
		let n = self.hist.items.len();
		if n == 0
		{
			return;
		}

		let (bs, i) = match self.hist.pos
		{
			Some(i) => (self.hist.items[i].chars().count(), cmp::max(i, 1) - 1),
			None => (self.ln.len(), n - 1),
		};

		let ln = &self.hist.items[i];
		let blank = ' '.to_string().repeat((self.offset + bs) - self.cur);
		let erase = '\x08'.to_string().repeat(bs);
		print!("{}{}{}", blank, erase, ln);
		self.cur = self.offset + ln.chars().count();
		self.hist.pos = Some(i);
	}


	// Update history
	fn histupdate(&mut self)
	{
		if let Some(i) = self.hist.pos
		{
			self.ln = self.hist.items[i].chars().collect();
			self.hist.pos = None;
		}
	}


	// Input
	pub fn input(&mut self, prompt: &str) -> Option<String>
	{
		print!("{}", prompt);
		self.offset = offset_from_prompt(prompt);
		self.cur = self.offset;
		self.ln = Vec::with_capacity(80);
		let mut parser = Parser::new();

		while let Some(c) = crate::libcore::io::stdin().readchar()
		{
			match c
			{
				// End of text
				'\x03' =>
				{
					println!();
					return Some(String::new());
				}

				// End of transmission
				'\x04' =>
				{
					println!();
					return None;
				},

				// New line
				'\n' =>
				{
					self.compupdate();
					self.histupdate();
					println!();
					return Some(self.ln.iter().collect());
				},

				c =>
				{
					for b in c.to_string().as_bytes()
					{
						parser.advance(self, *b);
					}
				}
			}
		}

		None
	}


	// New
	pub fn new() -> Self
	{
		Self
		{
			// Create a new instance of Completion
			compl: Completion::new(),

			// Create a new instance of History
			hist: History::new(),

			// Set offset to 0
			offset: 0,

			// Set cursor position to 0
			cur: 0,

			// Set line width to 80
			ln: Vec::with_capacity(80),
		}
	}
}


// Offset from prompt
fn offset_from_prompt(s: &str) -> usize
{
	let mut parser = Parser::new();
	let mut offset = Offset(0);

	for b in s.bytes()
	{
		parser.advance(&mut offset, b);
	}

	offset.0
}


// Empty completer function
fn nullcompleter(_ln: &str) -> Vec<String>
{
	Vec::new()
}
