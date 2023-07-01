// src/user/ut.rs
//
// The ut command will print the the amount of time that the system has been online since its last restart.

/*
	IMPORTS
*/

use crate::{println, sys::sc};

pub fn main(_args: &[&str]) -> crate::user::shell::XCode
{
	println!("UPTIME: {:.6}", sc::uptime());
	crate::user::shell::XCode::CMD_SUCCESS
}
