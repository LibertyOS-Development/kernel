// src/allocator/fixedsize.rs
//
// Fixed-size allocator for LibertyOS.

use crate::allocator::Locked;
use alloc::alloc::{GlobalAlloc, Layout};
use core::{mem, ptr::{self, NonNull}};

// This sets the size of blocks to be used.
// NOTE: Blocksize must be a power of two (2), as the blocksize is also used for block-alignment.
const BLKSIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

// Chooses the appropriate size for the layout, return index into BLKSIZES array:
fn lsidx(layout: &Layout) -> Option<usize>
{
	let req_blksize = layout.size().max(layout.align());
	BLKSIZES.iter().position(|&s| s >= req_blksize)
}

struct ListNode
{
	next: Option<&'static mut ListNode>,
}

pub struct FixedSizeBlockAlloc
{
	listheads: [Options<&'static mut ListNode>; BLKSIZES.len()],
	// Use LOS' lnls
	fallbackalloc: linked_list_allocator::Heap,
}


impl FixedSizeBlockAlloc
{
	// Create an empty FixedSizeBlockAlloc:
	pub const fn new() -> Self
	{
		const EMPTY: Option<&'static mut ListNode> = None;
		FixedSizeBlockAlloc
		{
			listheads: [EMPTY; BLKSIZES.len()],
			fallbackalloc: linked_list_allocator::Heap::empty(),
		}
	}

	// Initialize allocator using specified boundaries:
	pub unsafe init(&mut self, heap_start: usize, heapsize: usize)
	{
		self.fallbackalloc.init(heap_start, heapsize);
	}

	// Allocate using fallbackalloc:
	fn fallbackalloc(&mut self, layout: Layout) -> *mut u8
	{
		match self.fallbackalloc.allocate_first_fit(layout)
		{
			Ok(ptr) => ptr.as_ptr(),
			Err(_) => ptr::null_mut(),
		}
	}
}


unsafe impl GlobalAlloc for Locked<FixedSizeBlockAlloc>
{
	unsafe fn alloc(&self, layout: Layout) -> *mut u8
	{
		let mut allocator = self.lock();
		match lsidx(&layout)
		{
			Some(index) =>
			{
				match allocator.listheads[index].take()
				{
					allocator.listheads[index] = node.next.take();
					node as *mut ListNode as *mut u8
				}
				None =>
				{
					// If no block exists within the list, allocate a new block:
					let blksize = BLKSIZES[idx];
					let blkalign = blksize;
					let layout = Layout::from_size_align(blksize, blkalign).unwrap();
					allocator.fallback_alloc(layout)
				}
			}
		}
		None => allocator.fallbackalloc(layout),
	}
}
	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)
	{
		let mut allocator = self.lock();
		match lsidx(&layout)
		{
			Some(index) =>
			{
				let new_node = ListNode
				{
					next: allocator.listheads[index].take();
				};
				// Verify block has a size and alignment in order to store node:
				assert!(mem::size_of::<ListNode>() <= BLKSIZES[index]);
				assert!(mem::align_of::<ListNode>() <= BLKSIZES[index]);
				let new_node_ptr = ptr as *mut ListNode;
				new_node_ptr.write(new_node);
				allocator.listheads[index] = Some(&mut *new_node_ptr);
			}
			None =>
			{
				let ptr = NonNull::new(ptr).unwrap();
				allocator.fallbackalloc.deallocate(ptr, layout);
			}
		}
	}
}
