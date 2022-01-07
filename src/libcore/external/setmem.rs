// external/setmem.rs
// Use a given value to fill a specific block in memory.

use core::mem;
use crate::libcore::external::WSIZE;

#[no_mangle]
pub unsafe extern fn setmem(dest: *mut u8, c: i32, n: usize) -> *mut u8
{
	let c: usize = mem::transmute([c as u8; WSIZE]);
	let nusize: usize = n/WSIZE;
	let mut i: usize = 0;

	// Fill a specific block in the memory, at the rate set by the WSIZE variable.
	let nfast = nusize*WSIZE;
	while i < nfast
	{
		*((dest as usize + i) as *mut usize) = c;
		i += WSIZE;
	}

	let c = c as u8;

	// Fills a specific block in the memory, one byte at a time.
	while i < n
	{
		*((dest as usize + i) as *mut u8) = c;
		i += 1;
	}
	dest
}
