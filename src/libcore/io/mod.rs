// src/io/mod.rs
//
// This is the mod.rs file for the io module.


/*
	IMPORTS
*/

use crate::libcore::io::{stderr::Stderr, stdin::Stdin, stdout::Stdout};



pub mod stderr;
pub mod stdin;
pub mod stdout;



// Stderr
pub fn stderr() -> Stderr
{
	Stderr::new()
}


// Stdin
pub fn stdin() -> Stdin
{
	Stdin::new()
}


// Stdout
pub fn stdout() -> Stdout
{
	Stdout::new()
}
