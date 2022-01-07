// src/allocator/fixedsize.rs
//
// Fixed-size allocation for LibertyOS.

use crate::libcore::allocator::Locked;
use alloc::alloc::{GlobalAlloc, Layout};
use core::{mem, ptr::{self, NonNull}};

const BLKSIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

// Set the blocksize to use:
// NOTE: Blocksize must be a power of two (2).

fn lsidx(layout: &Layout) -> Option<usize>
{
	let req_blksize = layout.size().max(layout.align());
	BLKSIZES.iter().position(|&s| s >= req_blksize)
}

// Pair an appropriate blocksize for the specific layout in question. This returns an index into BLKSIZES.
struct ListNode
{
	next: Option<&'static mut ListNode>,
}

pub struct FixedSizeBlockAlloc
{
	listheads: [Option<&'static mut ListNode>; BLKSIZES.len()],
	fballoc: linked_list_allocator::Heap,
}

impl FixedSizeBlockAlloc
{
	// Create a new, empty FixedSizeBlockAlloc:
	pub const fn new() -> Self
	{
		const EMPTY: Option<&'static mut ListNode> = None;
		FixedSizeBlockAlloc
		{
			listheads: [EMPTY; BLKSIZES.len()],
			fballoc: linked_list_allocator::Heap::empty(),
		}
	}

	// Initialize allocator, using specified heap-bounds:
	pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize)
	{
		self.fballoc.init(heap_start, heap_size);
	}

	// Allocate with fballoc (fallback allocator):
	fn fballoc(&mut self, layout: Layout) -> *mut u8
	{
		match self.fballoc.allocate_first_fit(layout)
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
					Some(node) =>
					{
						allocator.listheads[index] = node.next.take();
						node as *mut ListNode as *mut u8
					}
					None =>
					{
						// If no block exists in list, allocate a new block.
						let blksize = BLKSIZES[index];
						let blkalign = blksize;
						let layout = Layout::from_size_align(blksize, blkalign).unwrap();
						allocator.fballoc(layout)
					}
				}
			}
			None => allocator.fballoc(layout),
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
					next: allocator.listheads[index].take(),
				};

				// Verify that the block has a size and an alignment.
				assert!(mem::size_of::<ListNode>() <= BLKSIZES[index]);
				assert!(mem::align_of::<ListNode>() <= BLKSIZES[index]);
				let new_node_ptr = ptr as *mut ListNode;
				new_node_ptr.write(new_node);
				allocator.listheads[index] = Some(&mut *new_node_ptr);
			}
			None =>
			{
				let ptr = NonNull::new(ptr).unwrap();
				allocator.fballoc.deallocate(ptr, layout);
			}
		}
	}
}
