use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use linked_list_allocator::LockedHeap;
use x86_64::{structures::paging::{mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB}, VirtAddr};

pub mod bump;
pub mod fixedsize;
pub mod lnls;

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap(mapper: &mut impl Mapper<Size4KiB>, frame_allocator: &mut impl FrameAllocator<Size4KiB>) -> Result<(), MapToError<Size4KiB>>
{
	let page_range =
	{
	        let heap_start = VirtAddr::new(HEAP_START as u64);
       		let heap_end = heap_start + HEAP_SIZE - 1u64;
       		let heap_start_page = Page::containing_address(heap_start);
     		let heap_end_page = Page::containing_address(heap_end);
       		Page::range_inclusive(heap_start_page, heap_end_page)
   	};

  	for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe
	{
		mapper.map_to(page, frame, flags, frame_allocator)?.flush()
	};
    }

	unsafe
	{
       		ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    	}

	Ok(())
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
