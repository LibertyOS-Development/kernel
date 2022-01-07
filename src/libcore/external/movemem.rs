// external/movemem.rs
// This is used to move bytes from one point in the memory to another, while allowing for areas of memory to overlap.

use crate::libcore::external::WSIZE;

#[no_mangle]
pub unsafe extern fn movemem(dest: *mut u8, src: *const u8, n: usize) -> *mut u8
{
	if src < dest as *const u8
	{
		// Moves the amount of bytes that has been set by the WSIZE variable.
		let nusize: usize = n/WSIZE;
		let mut i: usize = nusize*WSIZE;
		while i != 0
		{
			i -= WSIZE;
			*((dest as usize + i) as *mut usize) = *((src as usize + i) as *const usize);
		}
		let mut i: usize = n;

		// Moves bytes individually.
		while i != nusize*WSIZE
		{
			i -= 1;
			*((dest as usize + i) as *mut u8) = *((src as usize + i) as *const u8);
		}
	}
	else
	{
		let nusize: usize = n/WSIZE;
		let mut i: usize = 0;

		// Moves the amount of bytes that has been set by the WSIZE variable.
		let nfast = nusize*WSIZE;
		while i < nfast
		{
			*((dest as usize + i) as *mut usize) = *((src as usize + i) as *const usize);
			i += WSIZE;
		}

		// Moves bytes individually.
		while i < n
		{
			*((dest as usize + i) as *mut u8) = *((src as usize + i) as *const u8);
			i += 1;
		}
	}
	dest
}
