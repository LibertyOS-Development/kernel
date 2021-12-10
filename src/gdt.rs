#![allow(deprecated)]

use x86_64::VirtAddr;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use lazy_static::lazy_static;

pub const DOUBLEFAULT_IST_IDX: u16 = 0;

lazy_static!
{
	static ref TSS: TaskStateSegment = {
		let mut tss = TaskStateSegment::new();
		tss.interrupt_stack_table[DOUBLEFAULT_IST_IDX as usize] = {
			const STACKSIZE: usize = 4096 * 5;
			static mut STACK: [u8; STACKSIZE] = [0; STACKSIZE];
			let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
			let stack_stop = stack_start + STACKSIZE;
			stack_stop
		};
		tss
	};
}

lazy_static!
{
	static ref GDT: (GlobalDescriptorTable, Selectors) = {
		let mut gdt = GlobalDescriptorTable::new();
		let codesel = gdt.add_entry(Descriptor::kernel_code_segment());
		let tsssel = gdt.add_entry(Descriptor::tss_segment(&TSS));
		(
			gdt,
			Selectors
			{
				codesel,
				tsssel,
			 },
		)
	};
}

struct Selectors
{
	codesel: SegmentSelector,
	tsssel: SegmentSelector,
}


pub fn init()
{
	use x86_64::instructions::segmentation::set_cs;
	use x86_64::instructions::tables::load_tss;
	GDT.0.load();
	unsafe
	{
		set_cs(GDT.1.codesel);
		load_tss(GDT.1.tsssel);
	}
}
