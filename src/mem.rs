// src/mem.rs
//
// Basic memory management functions.

#![allow(dead_code)]
#![allow(deprecated)]
#![allow(unused_features)]


/*
	IMPORTS
*/

use core::sync::atomic::{AtomicU64, Ordering};
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use bootloader::BootInfo;
use x86_64::{PhysAddr, VirtAddr};
use x86_64::instructions::interrupts;
use x86_64::structures::paging::{FrameAllocator, Mapper, OffsetPageTable, Page, page::PageRangeInclusive, PageTable, PhysFrame, Size4KiB, Translate};

use crate::serprint;


// Physical memory offset
pub static mut PMEM_OFFSET: u64 = 0;

// Memory map
pub static mut MEMMAP: Option<&MemoryMap> = None;

// Memory size
pub static MEMSIZE: AtomicU64 = AtomicU64::new(0);



// Initialization
pub fn init(bootinfo: &'static BootInfo)
{
	interrupts::without_interrupts(||
	{
		let mut memsize = 0;

		for region in bootinfo.memory_map.iter()
		{
			let start_address = region.range.start_addr();
			let end_address = region.range.end_addr();

			memsize += end_address - start_address;
			serprint!("[INFO] MEM [{:#016X}-{:#016X}] {:?}\n", start_address, end_address, region.region_type);
		}

		serprint!("[INFO] MEM: {} KB\n", memsize >> 10);
		MEMSIZE.store(memsize, Ordering::Relaxed);

		unsafe
		{
			PMEM_OFFSET = bootinfo.physical_memory_offset
		};

		unsafe
		{
			MEMMAP.replace(&bootinfo.memory_map)
		};

		let mut mapper = unsafe
		{
			mapper(VirtAddr::new(PMEM_OFFSET))
		};

		let mut framealloc = unsafe
		{
			BootInfoFrameAllocator::init(&bootinfo.memory_map)
		};

		crate::libcore::allocator::init_heap(&mut mapper, &mut framealloc)
			.expect("[ERR] FAILED TO INITALIZE HEAP");
	});
}


unsafe fn active_lvl4_tab(physmem_offset: VirtAddr) -> &'static mut PageTable
{
	use x86_64::registers::control::Cr3;
	let(lvl4_tab_frame, _) = Cr3::read();
	let phys = lvl4_tab_frame.start_address();
	let virt = physmem_offset + phys.as_u64();
	let pagetab_ptr: *mut PageTable = virt.as_mut_ptr();
	&mut *pagetab_ptr
}


// Deallocate pages
pub fn p_dealloc(address: u64, size: u64)
{
	let mut mapper = unsafe
	{
			crate::mem::mapper(VirtAddr::new(crate::mem::PMEM_OFFSET))
	};

	let pages: PageRangeInclusive<Size4KiB> =
	{
		let page_start = Page::containing_address(VirtAddr::new(address));
		let page_end = Page::containing_address(VirtAddr::new(address + size));
		Page::range_inclusive(page_start, page_end)
	};

	for page in pages
	{
		if let Ok((_frame, mapping)) = mapper.unmap(page)
		{
			mapping.flush();
		}
		else
		{
			unimplemented!();
		}
	}
}


// Physical-address to virtual-address
pub fn ptov(address: PhysAddr) -> VirtAddr
{
	VirtAddr::new(address.as_u64() + unsafe
	{
		PMEM_OFFSET
	})
}


// This is a FrameAllocator that will always return a value of "None".
pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator
{
	fn allocate_frame(&mut self) -> Option<PhysFrame>
	{
		None
	}
}


// This is a FrameAllocator that will return any usable frames from the memory map of the bootloader.
pub struct BootInfoFrameAllocator
{
	memmap: &'static MemoryMap,
	next: usize,
}


impl BootInfoFrameAllocator
{
	// This creates a FrameAllocator from a memory map.
	pub unsafe fn init(memmap: &'static MemoryMap) -> Self
	{
		BootInfoFrameAllocator
		{
			memmap,
			next: 0,
		}
	}

	// This returns an iterator over usable frames, as specified by the memory map.
	fn usableframes(&self) -> impl Iterator<Item = PhysFrame>
	{
		// This figures out which regions, from the memory map, are usable.
		let regions = self.memmap.iter();
		let usableregions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);

		// This maps each region to its respective address range.
		let addr_ranges = usableregions.map(|r| r.range.start_addr()..r.range.end_addr());

		// This transforms to an iterator of the frame start addresses.
		let frameaddr = addr_ranges.flat_map(|r| r.step_by(4096));

		// This creates a PhysFrame type from each start address.
		frameaddr.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
	}
}


unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator
{
	fn allocate_frame(&mut self) -> Option<PhysFrame>
	{
		let frame = self.usableframes().nth(self.next);
		self.next += 1;
		frame
	}
}


// Mapper
pub unsafe fn mapper(pmem_offset: VirtAddr) -> OffsetPageTable<'static>
{
	let lvl4_tab = active_lvl4_tab(pmem_offset);
	OffsetPageTable::new(lvl4_tab, pmem_offset)
}


// Memory size
pub fn memsize() -> u64
{
	MEMSIZE.load(Ordering::Relaxed)
}


// Virtual to physical address
pub fn vtop(address: VirtAddr) -> Option<PhysAddr>
{
	let mapper = unsafe
	{
		mapper(VirtAddr::new(PMEM_OFFSET))
	};

	mapper.translate_addr(address)
}
