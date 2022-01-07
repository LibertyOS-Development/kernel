// external/cpmem.rs
// This is used to copy bytes from one area of memory to another.

use crate::libcore::external::WSIZE;

#[no_mangle]
pub unsafe extern fn cpmem(dest: *mut u8, src: *const u8, n: usize) -> *mut u8
{
	let nusize: usize = n/WSIZE;
	let mut i: usize = 0;

	let nfast = nusize*WSIZE;
	while i < nfast
	{
		*((dest as usize + i) as *mut usize) = *((src as usize + i) as *const usize);
		i += WSIZE;
	}

	while i < n
	{
		*((dest as usize + i) as *mut u8) = *((src as usize + i) as *const u8);
		i += 1;
	}
	dest
}
