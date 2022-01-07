// src/allocator/lnls.rs
//
// Linked-list allocator for LibertyOS.

use crate::libcore::allocator::{alignup, Locked};
use alloc::alloc::{GlobalAlloc, Layout};
use core::{mem, ptr};

struct ListNode
{
	size: usize,
	next: Option<&'static mut ListNode>,
}

impl ListNode
{
	const fn new(size: usize) -> Self
	{
		ListNode
		{
			size,
			next: None
		}
	}

	fn addr_start(&self) -> usize
	{
		self as *const Self as usize
	}

	fn addr_end(&self) -> usize
	{
		self.addr_start() + self.size
	}
}

pub struct LnLsAlloc
{
	head: ListNode,
}

impl LnLsAlloc
{
	// Create a new, empty LnLsAlloc:
	pub const fn new() -> Self
	{
		Self
		{
			head: ListNode::new(0),
		}
	}

	// Initialize allocator with specified heap bounds:
	pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize)
	{
		self.add_freergn(heap_start: usize, heap_size: usize)
	}

	// Add specified memreg to front of the list:
	unsafe fn add_freergn(&mut self, addr: usize, size: usize)
	{
			// Verify that freed region is able to hold ListNode:
			assert_eq!(alignup(addr, mem::align_of::<ListNode>()), addr);
			assert!(size >= mem::size_of::<ListNode>());

			// Create new list-node, append to start of list:
			let mut node = ListNode::new(size);
			node.next = self.head.next.take();
			let node_ptr = addr as *mut ListNode;
			node_ptr.write(node);
			self.head.next = Some(&mut *node_ptr)
	}

	// Locate a free region, matching the specified size and alignment, and removes said region from list:
	// NOTE: This will return a tuple, containing the list-node, and the start address of the allocation.

	fn findrgn(&mut self, size: usize, align: usize) -> Option<(&'static mut ListNode, usize)>
	{
		// Refers to current list-node:
		// NOTE: Each iteration will update the list-node.
		let mut current = &mut self.head;
		// Locate a memreg with sufficient size from linked-list.
		while let Some(ref mut region) = current.next
		{
			if let Ok(alloc_start) = Self::alloc_fromrgn(&region, size, align)
			{
				// If region is appropriate for allocation, the node is removed from the list.
				let next = region.next.take();
				let ret = Some((current.next.take().unwrap(), alloc_start));
				current.next = next;
				return ret;
			}
			else
			{
				// If the region is inappropriate, continue to the next region.
				current = current.next.as_mut().unwrap();
			}
		}
		// If no appropriate region is able to be located:
		None
	}

	// Attempt to allocate using a specific region, with a specifc size and alignment:
	fn alloc_fromrgn(region: &ListNode, size: usize, align: usize) -> Result<usize, ()>
	{
		let alloc_start = alignup(region.addr_start(), align);
		let alloc_end = alloc_start.checked_add(size).ok_or(())?;

		if alloc_end > region.addr_end()
		{
			// If memory region is too small:
			return Err(());
		}

		let excess_size = region.addr_end() - alloc_end;

		if excess_size > 0 && excess_size < mem::size_of::<ListNode>()
		{
			// If the remaining region is insufficient in size to hold a ListNode:
			return Err(());
		}

		// If memory region is viable for allocation:
		Ok(alloc_start)
	}

	// Modify specified layout so that the allocated memory region can also hold a ListNode:
	// NOTE: This will return a tuple, consisting of the adjusted size, and the alignment (size, align).
	fn sizealign(layout: Layout) -> (usize, usize)
	{
		let layout = layout
			.align_to(mem::align_of::<ListNode>())
			.expect("[ERR] FAILED TO ADJUST ALIGNMENT")
			.pad_to_align();
		let size = layout.size().max(mem::size_of::<ListNode>());
		(size, layout.align())
	}
}


unsafe impl GlobalAlloc for Locked<LnLsAlloc>
{
	unsafe fn alloc(&self, layout: Layout) -> *mut u8
	{
		// Make adjustment(s) to layout:
		let (size, align) = LnLsAlloc::sizealign(layout);
		let mut allocator = self.lock();

		if let Some((region, alloc_start)) = allocator.findrgn(size, align)
		{
			let alloc_end = alloc_start.checked_add(size).expect("[ERR] OVERFLOW");
			let excess_size = region.addr_end() - alloc_end;
			if excess_size > 0
			{
				allocator.add_freergn(alloc_end, excess_size);
			}
			alloc_start as *mut u8
		}
		else
		{
			ptr::null_mut()
		}
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)
	{
		// Make adjustment(s) to layout:
		let (size, _) = LnLsAlloc::sizealign(layout);
		self.lock().add_freergn(ptr as usize, size)
	}
}
