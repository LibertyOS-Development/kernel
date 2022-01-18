// src/gdt.rs
//
// Global descriptor table (GDT) functionality for the LibertyOS kernel.


#![allow(deprecated)]


/*
	IMPORTS
*/

use x86_64::{PrivilegeLevel, VirtAddr};
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, DescriptorFlags, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::instructions::segmentation::*;
use x86_64::instructions::tables::load_tss;
use lazy_static::lazy_static;



// Double-fault index
pub const DOUBLEFAULT_IST_IDX: u16 = 0;


// General protection fault index
pub const GEN_PROT_FAULT_ISTIDX: u16 = 2;


// Page-fault index
pub const PAGE_FAULT_ISTIDX: u16 = 1;


// Privilege TSS stack
pub static mut PRIV_TSS_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];


// Stack
pub static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];


// Stack size
const STACK_SIZE: usize = 0x2000;



// Lazy static wrapper around the TSS reference
lazy_static!
{
	static ref TSS: TaskStateSegment = {
		// Create new TSS
		let mut tss = TaskStateSegment::new();


		// Privilege stack table for the TSS
		tss.privilege_stack_table[0] =
		{
			static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
			VirtAddr::from_ptr(unsafe
			{
				&STACK
			}) + STACK_SIZE
		};


		// Double-fault index for the interrupt stack table of the TSS
		tss.interrupt_stack_table[DOUBLEFAULT_IST_IDX as usize] =
		{
			static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
			VirtAddr::from_ptr(unsafe
			{
				&STACK
			}) + STACK_SIZE
		};


		// Page-fault index for the interrupt stack table of the TSS
		tss.interrupt_stack_table[PAGE_FAULT_ISTIDX as usize] =
		{
			static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
			VirtAddr::from_ptr(unsafe
			{
				&STACK
			}) + STACK_SIZE
		};


		// General protection fault index for the interrupt stack table of the TSS
		tss.interrupt_stack_table[GEN_PROT_FAULT_ISTIDX as usize] =
		{
			static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
			VirtAddr::from_ptr(unsafe
			{
				&STACK
			}) + STACK_SIZE
		};


		// Return the fully-functional version of the TSS
		tss
	};
}


// Lazy static wrapper around the GDT static reference
lazy_static!
{
	pub static ref GDT: (GlobalDescriptorTable, Selectors) =
	{
		// Create new GDT
		let mut gdt = GlobalDescriptorTable::new();


		// Add TSS segment to the GDT
		let tss = gdt.add_entry(Descriptor::tss_segment(&TSS));

		// Add kernel code segment to the GDT
		let code = gdt.add_entry(Descriptor::kernel_code_segment());

		// Add kernel data segment to the GDT
		let data = gdt.add_entry(Descriptor::kernel_data_segment());

		// Add user code segment to the GDT
		let usercode = gdt.add_entry(Descriptor::user_code_segment());

		// Add user data segment to the GDT
		let userdata = gdt.add_entry(Descriptor::user_data_segment());

		(gdt, Selectors
		{
			tss,
			code,
			data,
			usercode,
			userdata
		})
	};
}


// Selectors struct
// NOTE: This struct was originally private, but, in order for the kernel to compile with the load_tss line in the init function, this struct had to be public
pub struct Selectors
{
	code: SegmentSelector,
	data: SegmentSelector,
	tss: SegmentSelector,
	pub usercode: SegmentSelector,
	pub userdata: SegmentSelector,
}


// Initialization
pub fn init()
{
	GDT.0.load();

	unsafe
	{
		CS::set_reg(GDT.1.code);
		DS::set_reg(GDT.1.data);
		load_tss(GDT.1.tss);
	}
}
