// src/libcore/allocator/mod.rs
//
// This is the mod.rs file for the libcore::allocator module.

/*
	IMPORTS
*/

use alloc::{alloc::{GlobalAlloc, Layout}, slice::SliceIndex, sync::Arc, vec::Vec, vec};
use core::{cmp, ops::{Index, IndexMut}, ptr::null_mut};
use linked_list_allocator::LockedHeap;
use spin::Mutex;
use x86_64::{structures::paging::{mapper::MapToError, FrameAllocator, Mapper, Page, page::PageRangeInclusive, PageTableFlags, Size4KiB}, VirtAddr};

use crate::print;

// Bump allocation
pub mod bump;

// Fixed-size allocation
pub mod fixedsize;

// Linked-list allocation
pub mod lnls;



pub const HEAP_START: usize = 0x_4444_4444_0000;


#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap(mapper: &mut impl Mapper<Size4KiB>, frame_allocator: &mut impl FrameAllocator<Size4KiB>) -> Result<(), MapToError<Size4KiB>>
{
	// Allocate half of available memory to the heap.
	// NOTE: Memory allocated to the heap cannot exceed 16MB.
	let hsize = cmp::min(crate::mem::memsize() / 2, 16 << 20);

	let pages =
	{
		let hstart = VirtAddr::new(HEAP_START as u64);
		let hend = hstart + hsize - 1u64;
		let hstartpage = Page::containing_address(hstart);
		let hendpage = Page::containing_address(hend);

		Page::range_inclusive(hstartpage, hendpage)
	};

	let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

	for page in pages
	{
		let frame = frame_allocator.allocate_frame().ok_or(MapToError::FrameAllocationFailed)?;

		unsafe
		{
			mapper.map_to(page, frame, flags, frame_allocator)?.flush();
		}
	}

	unsafe
	{
		ALLOCATOR.lock().init(HEAP_START, hsize as usize);
	}

	Ok(())
}



// Allocate pages
pub fn palloc(address: u64, size: u64)
{
	let mut mapper = unsafe
	{
		crate::mem::mapper(VirtAddr::new(crate::mem::PMEM_OFFSET))
	};

	let mut framealloc = unsafe
	{
		crate::mem::BootInfoFrameAllocator::init(crate::mem::MEMMAP.unwrap())
	};

	let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;

	let pages =
	{
		let spage = Page::containing_address(VirtAddr::new(address));
		let epage = Page::containing_address(VirtAddr::new(address + size));
		Page::range_inclusive(spage, epage)
	};

	for page in pages
	{
		let frame = framealloc.allocate_frame().unwrap();

		unsafe
		{
			if let Ok(mapping) = mapper.map_to(page, frame, flags, &mut framealloc)
			{
				mapping.flush();
			}
			else
			{
				print!("[ERR] UNABLE TO MAP {:?}", page);
			}
		}
	}
}


// Deallocate pages
pub fn pdealloc(address: u64, size: u64)
{
	let mut mapper = unsafe
	{
		crate::mem::mapper(VirtAddr::new(crate::mem::PMEM_OFFSET))
	};

	let pages: PageRangeInclusive<Size4KiB> =
	{
		let spage = Page::containing_address(VirtAddr::new(address));
		let epage = Page::containing_address(VirtAddr::new(address + size));

		Page::range_inclusive(spage, epage)
	};

	for page in pages
	{
		if let Ok((_frame, mapping)) = mapper.unmap(page)
		{
			mapping.flush();
		}
		else
		{
			print!("[ERR] COULD NOT DEALLOCATE {:?}", page);
		}
	}
}


pub struct Dummy;

unsafe impl GlobalAlloc for Dummy
{
	unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
	null_mut()
}

unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout)
{
	panic!("[ERR] DEALLOC SHOULD NOT BE CALLED")
	}
}


// This is a wrapper around spin::Mutex, in order to allow trait implementations.
pub struct Locked<A>
{
	inner: spin::Mutex<A>,
}

impl<A> Locked<A>
{
	pub const fn new(inner: A) -> Self
	{
		Locked
		{
			inner: spin::Mutex::new(inner),
		}
	}

	pub fn lock(&self) -> spin::MutexGuard<A>
	{
		self.inner.lock()
	}
}

// This aligns the specified "address", upwards, to "align". In order to use this function, "align" must have a value that is a power of two (2).
fn alignup(address: usize, align: usize) -> usize
{
	(address + align - 1) & !(align - 1)
}


// Free memory
pub fn memfree() -> usize
{
	ALLOCATOR.lock().free()
}


// Memory size
pub fn memsize() -> usize
{
	ALLOCATOR.lock().size()
}


// Used memory
pub fn memused() -> usize
{
	ALLOCATOR.lock().free()
}


// PhysicalBuffer struct
#[derive(Clone)]
pub struct PhysicalBuffer
{
	buffer: Arc<Mutex<Vec<u8>>>,
}


// Implementation of the PhysicalBuffer struct
impl PhysicalBuffer
{
	// Address
	pub fn address(&self) -> u64
	{
		physaddr(&self.buffer.lock()[0])
	}


	// From
	pub fn from(vec: Vec<u8>) -> Self
	{
		let bufferlen = vec.len() - 1;
		let memlen = physaddr(&vec[bufferlen]) - physaddr(&vec[0]);

		if bufferlen == memlen as usize
		{
			Self
			{
				buffer: Arc::new(Mutex::new(vec))
			}
		}
		else
		{
			// If there is an error, clone the vector and retry
			Self::from(vec.clone())
		}
	}


	// New
	pub fn new(len: usize) -> Self
	{
		Self::from(vec![0; len])
	}
}


// Physical address
pub fn physaddr(ptr: &u8) -> u64
{
	let rxptr = ptr as *const u8;
	let virtaddr = VirtAddr::new(rxptr as u64);
	let physaddr = crate::mem::vtop(virtaddr).unwrap();
	physaddr.as_u64()
}
