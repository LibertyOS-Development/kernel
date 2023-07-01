// src/sys/sc/svc.rs - (Easily the most annoying path in all of LOS)
//
// Services for the libcore::sys::sc module.

/*
	IMPORTS
*/

use alloc::vec;

use crate::{fs::FileIO, sys::sc::FileInfo};


// Duplicate
pub fn dp(original: usize, new: usize) -> isize
{
	if let Some(file) = crate::sys::proc::fh(original)
	{
		crate::sys::proc::fh_update(new, file);
		return new as isize;
	}
	-1
}

// Info
pub fn info(path: &str, info: &mut FileInfo) -> isize
{
	if let Some(res) = crate::fs::info(path)
	{
		*info = res;
		0
	}
	else
	{
		-1
	}
}


// Read
pub fn rd(handle: usize, buffer: &mut [u8]) -> isize
{
	if let Some(mut file) = crate::sys::proc::fh(handle)
	{
		if let Ok(bytes) = file.read(buffer)
		{
			crate::sys::proc::fh_update(handle, file);
			return bytes as isize;
		}
	}
	-1
}


// Real-time
pub fn rt() -> f64
{
	crate::clock::realtime()
}


// Sleep
pub fn sl(sec: f64)
{
	crate::time::sleep(sec);
}


// Uptime
pub fn ut() -> f64
{
	crate::clock::uptime()
}


// Write
pub fn wr(handle: usize, buffer: &mut [u8]) -> isize
{
	if let Some(mut file) = crate::sys::proc::fh(handle)
	{
		if let Ok(bytes) = file.write(buffer)
		{
			crate::sys::proc::fh_update(handle, file);
			return bytes as isize;
		}
	}
	-1
}
