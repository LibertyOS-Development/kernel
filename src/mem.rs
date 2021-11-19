\use x86_64::{structures::paging::PageTable, PhysAddr, VirtAddr};


pub unsafe fn translate_address(addr: VirtAddr, physmem_offset: VirtAddr) -> Option<PhysAddr>
{
	translate_addr_inner(addr, physmem_offset)
}

fn translate_addr_inner(addr: VirtAddr, physmem_offset: VirtAddr) -> Option<PhysAddr>
{
	use x86_64::structures::paging::page_table::FrameError;
	use x86_64::registers::control::Cr3;

	let (level_4_table_frame, _) = Cr3::read();

	let tabidx = [
		addr.p4idx(),
		addr.p3idx(),
		addr.p2idx(),
		addr.p1idx(),
	];
	let mut frame = level_4_table_frame;

	for &idx in &tabidx
	{
		let virt = physmem_offset + frame.start_address().as_u64();
		let tabptr: *const PageTable = virt.as_ptr();
		let tab = unsafe
		{
			&*tabptr
		};
		let entry = &tab[idx];
		frame = match entry.frame()
		{
			Ok(frame) => frame,
			Err(FrameError::FrameNotPresent) => return None,
			Err(FrameError::HugeFrame) => panic!("[ERR] HUGE PAGES ARE UNSUPPORTED"),
		};
	}

	Some(frame.start_address() + u64::from(addr.page_offset()))
}

pub unsafe fn active_lvl4_tab(physmem_offset: VirtAddr) -> &'static mut PageTable
{
	use x86_64::registers::control::Cr3;
	let (level_4_table_frame, _) = Cr3::read();
	let phys = level_4_table_frame.start_address();
	let virt = physmem_offset + phys.as_u64();
	let pagetab_ptr: *mut PageTable = virt.as_mut_ptr();
	&mut *pagetab_ptr
}
