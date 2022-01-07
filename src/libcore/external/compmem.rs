// external/compmem.rs
// Compare two specific blocks from the system memory.

use crate::libcore::external::WSIZE;

#[no_mangle]
pub unsafe extern fn compmem(s1: *const u8, s2: *const u8, n: usize) -> i32
{
	let nusize: usize = n/WSIZE;
	let mut i: usize = 0;
	let nfast = nusize*WSIZE;
	while i < nfast
	{
		let a = *((s1 as usize + i) as *const usize);
		let b = *((s2 as usize + i) as *const usize);
		if a != b
		{
			let n: usize = i + WSIZE;

			// Locate the unequal byte
			while i < n
			{
				let a = *((s1 as usize + i) as *const u8);
				let b = *((s2 as usize + i) as *const u8);
				if a != b
				{
					return a as i32 - b as i32;
				}
				i += 1;
			}
		}
		i += WSIZE;
	}

	while i < n
	{
		let a = *((s1 as usize + i) as *const u8);
		let b = *((s2 as usize + i) as *const u8);
		if a != b
		{
			return a as i32 - b as i32;
		}
		i += 1;
	}
	0
}
