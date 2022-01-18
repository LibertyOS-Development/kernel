// src/libcore/user/shell.rs
//
// This is the builtin shell for LibertyOS.

#![allow(non_camel_case_types)]


/*
	IMPORTS
*/

use alloc::{format, string::String, vec::Vec};

use crate::{print, println, {libcore::sys::console::Style}};


// Autocompletion commands
pub const AUTOCMD: [&str; 1] = [
	"help"
	];


// XCode enumeration
#[repr(u8)]
#[derive(PartialEq)]
pub enum XCode
{
	// Success
	CMD_SUCCESS = 0,

	// Unknown
	CMD_UNK = 1,

	// Error
	CMD_ERR = 2,

	// Exit shell
	SHEX = 255,
}


// Execute
pub fn exec(cmd: &str) -> XCode
{
	let mut args = splargs(cmd);
	let mut redir = false;
	let mut n = args.len();
	let mut i = 0;

	loop
	{
		if i == n
		{
			break;
		}

		let mut fatarrow = false;
		let mut thinarrow = false;
		let mut lefthandle;

		if crate::rgx::Rgx::new("<=+").ismatch(args[i])
		{
			fatarrow = true;
			lefthandle = 0;
		}
		else if crate::rgx::Rgx::new("\\d*=+>").ismatch(args[i])
		{
			fatarrow = true;
			lefthandle = 1;
		}
		else if crate::rgx::Rgx::new("\\d*-*>\\d*").ismatch(args[i])
		{
			thinarrow = true;
			lefthandle = 1;
		}
		else
		{
			i += 1;
			continue;
		}

		let s = args[i].chars().take_while(|c| c.is_numeric()).collect::<String>();
		if let Ok(h) = s.parse()
		{
			lefthandle = h;
		}

		if fatarrow
		{
			redir = true;
			if i == n - 1
			{
				println!("[ERR] UNABLE TO PARSE PATH");
				return XCode::CMD_ERR;
			}

			let path = args[i + 1];
			if crate::libcore::fs::reopen(path, lefthandle).is_err()
			{
				println!("[ERR] UNABLE TO OPEN PATH");
				return XCode::CMD_ERR;
			}

			args.remove(i);
			args.remove(i);
			n -= 2;
		}
		else if thinarrow
		{
			println!("[ERR] UNABLE TO PARSE ARROW");
			return XCode::CMD_ERR;
		}
	}

	let res = match args[0]
	{
		"help" => unimplemented!(),
		cmd =>
		{
			if crate::libcore::sys::proc::spawn(cmd).is_ok()
			{
				XCode::CMD_SUCCESS
			}
			else
			{
				XCode::CMD_UNK
			}
		}
	};


	if redir
	{
		for i in 0..3
		{
			crate::libcore::fs::reopen("/dev/console", i).ok();
		}
	}

	res
}


// Main
pub fn main(args: &[&str]) -> XCode
{
	match args.len()
	{
		1 =>
		{
			run()
		},

		2 =>
		{
			let pname = args[1];
			if let Ok(contents) = crate::libcore::fs::read_to_str(pname)
			{
				for ln in contents.split('\n')
				{
					if !ln.is_empty()
					{
						exec(ln);
					}
				}

				XCode::CMD_SUCCESS
			}
			else
			{
				println!("[ERR] FILE NOT FOUND: '{}'", pname);
				XCode::CMD_ERR
			}
		},

		_ =>
		{
			XCode::CMD_ERR
		},
	}
}

// Prompt string function
pub fn promptstr(success: bool) -> String
{
	let csicol = Style::color("Magenta");
	let csierr = Style::color("Red");
	let csires = Style::reset();

	format!("{}LOS:{} ", if success
	{
		csicol
	}
	else
	{
		csierr
	},

	csires)
}


// Run
pub fn run() -> crate::libcore::user::shell::XCode
{
	println!();

	let mut prompt = crate::libcore::sys::prompt::Prompt::new();
	let histfile = "~/.sh-hist";
	prompt.hist.load(histfile);
	prompt.compl.set(&shcomp);

	let mut success = true;
	while let Some(cmd) = prompt.input(&promptstr(success))
	{
		match exec(&cmd)
		{
			XCode::CMD_SUCCESS =>
			{
				success = true;
			},

			XCode::SHEX =>
			{
				break;
			},

			_ =>

			{
				success = false;
			},
		}

		prompt.hist.add(&cmd);
		prompt.hist.save(histfile);
		crate::libcore::sys::console::drain();
		println!();
	}

	// Clears screen, move cursor to top of display
	print!("\x1b[2J\x1b[1;1H");
	XCode::CMD_SUCCESS
}


// Shell completion
pub fn shcomp(ln: &str) -> Vec<String>
{
	let mut items = Vec::new();

	let args = splargs(ln);
	let i = args.len() - 1;

	// Autocomplete command
	if args.len() == 1
	{
		for &cmd in &AUTOCMD
		{
			if let Some(item) = cmd.strip_prefix(args[i])
			{
				items.push(item.into());
			}
		}
	}
	// Autocomplete path
	else
	{
		let pname = crate::libcore::fs::rpath(args[i]);
		let dname = crate::libcore::fs::dname(&pname);
		let fname = crate::libcore::fs::fname(&pname);
		let sep = if dname.ends_with('/')
		{
			""
		}
		else
		{
			"/"
		};

		if let Some(directory) = crate::libcore::fs::directory::Directory::open(dname)
		{
			for item in directory.items()
			{
				let name = item.name();
				if name.starts_with(fname)
				{
					let end = if item.isdir()
					{
						"/"
					}
					else
					{
						""
					};
					let path = format!("{}{}{}{}", dname, sep, name, end);
					items.push(path[pname.len()..].into());
				}
			}
		}
	}
	items
}


// Split arguements
pub fn splargs(cmd: &str) -> Vec<&str>
{
	let mut args: Vec<&str> = Vec::new();
	let mut i = 0;
	let mut n = cmd.len();
	let mut quotation = false;

	for (j, c) in cmd.char_indices()
	{
		if c == '#' && !quotation
		{
			// Throw out comments
			n = j;
			break;
		}
		else if c == ' ' && !quotation
		{
			if i != j
			{
				args.push(&cmd[i..j]);
			}

			i = j + 1;
		}
		else if c == '"'
		{
			quotation = !quotation;
			if !quotation
			{
				args.push(&cmd[i..j]);
			}
			i = j + 1;
		}
	}

	if i < n
	{
		if quotation
		{
			n -= 1;
		}
		args.push(&cmd[i..n]);
	}

	if n == 0 || cmd.ends_with(' ')
	{
		args.push("");
	}

	args
}
