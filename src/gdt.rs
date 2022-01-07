#![allow(deprecated)]

use x86_64::{PrivilegeLevel, VirtAddr};
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, DescriptorFlags, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::instructions::segmentation::*;
use lazy_static::lazy_static;

pub const DOUBLEFAULT_IST_IDX: u16 = 0;
const STACK_SIZE: usize = 0x2000;
pub static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
pub static mut PRIV_TSS_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

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
	static ref GDT: (GlobalDescriptorTable, [SegmentSelector; 5]) = {
		let mut gdt = GlobalDescriptorTable::new();
		let kernel_data_flags = DescriptorFlags::USER_SEGMENT | DescriptorFlags::PRESENT | DescriptorFlags::WRITABLE;
		let code_sel = gdt.add_entry(Descriptor::kernel_code_segment());
		let data_sel = gdt.add_entry(Descriptor::UserSegment(kernel_data_flags.bits()));
		let tss_sel = gdt.add_entry(Descriptor::tss_segment(&TSS));
		let user_datasel = gdt.add_entry(Descriptor::user_data_segment());
		let user_codesel = gdt.add_entry(Descriptor::user_code_segment());
		(
			gdt,
			[
				code_sel,
				data_sel,
				tss_sel,
				user_datasel,
				user_codesel
			]
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

	let stack = unsafe { &STACK as *const _ };
	let userstack = unsafe { &PRIV_TSS_STACK as *const _ };

	unsafe
	{
		set_cs(GDT.1[0]);
		load_ds(GDT.1[1]);
		load_tss(GDT.1[2]);
	}
}


#[inline(always)]
pub unsafe fn usermode_seg_set() -> (u16, u16)
{
	let (mut cs, mut ds) = (GDT.1[4], GDT.1[3]);
	cs.0 |= PrivilegeLevel::Ring3 as u16;
	ds.0 |= PrivilegeLevel::Ring3 as u16;
	load_ds(ds);
	(cs.0, ds.0)
}
