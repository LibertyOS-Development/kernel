// src/allocator/bump.rs
//
// LibertyOS' bump-allocator.

use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr;

use crate::libcore::allocator::{alignup, Locked};


pub struct BumpAlloc
{
	initheap: usize,
	endheap: usize,
	next: usize,
	allocations: usize,
}

impl BumpAlloc
{
	// Create a new bump-allocator.
	pub const fn new() -> Self
	{
		BumpAlloc
		{
			initheap: 0,
			endheap: 0,
			next: 0,
			allocations: 0,
		}
	}

	// Initialize the new bump-allocator, using the heap-boundaries that were specified.
	// Note: This is an unsafe function.

	pub unsafe fn init(&mut self, initheap: usize, heapsize: usize)
	{
		self.initheap = initheap;
		self.endheap = initheap.saturating_add(heapsize);
		self.next = initheap;
	}
}


unsafe impl GlobalAlloc for Locked<BumpAlloc>
{
	unsafe fn alloc(&self, layout: Layout) -> *mut u8
	{
		// Get a mutable reference.
		let mut bump = self.lock();

		let alloc_init = alignup(bump.next, layout.align());
		let alloc_end = match alloc_init.checked_add(layout.size())
		{
			Some(end) => end,
			None => return ptr::null_mut(),
		};

		if alloc_end > bump.endheap
		{
			// No memory remaining.
			ptr::null_mut()
		}
		else
		{
			bump.next = alloc_end;
			bump.allocations += 1;
			alloc_init as *mut u8
		}
	}

	unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout)
	{
		// Get a mutable reference.
		let mut bump = self.lock();
		bump.allocations -= 1;
		if bump.allocations == 0
		{
			bump.next = bump.initheap;
		}
	}
}
