#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(libertyos_kernel::testexec)]
#![reexport_test_harness_main = "testexec"]
#![allow(dead_code)]
#![allow(deprecated)]
#![allow(unused_features)]
#![allow(unused_variables)]

extern crate alloc;

use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use libertyos_kernel::println;

entry_point!(kernel_main);


fn kernel_main(bootinfo: &'static BootInfo) -> !
{
	use libertyos_kernel::mem::{self, BootInfoFrameAllocator};
	use libertyos_kernel::allocator;
	use x86_64::{structures::paging::Page, VirtAddr};

	println!("LIBERTY-OS");
	println!("KERNEL VERSION 0.11.1");
	libertyos_kernel::init();

	let physmem_offset = VirtAddr::new(bootinfo.physical_memory_offset);
	let mut mapper = unsafe
	{
		mem::init(physmem_offset)
	};
	let mut framealloc = unsafe
	{
		BootInfoFrameAllocator::init(&bootinfo.memory_map)
	};

	let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
	libertyos_kernel::mem::new_example_mapping(page, &mut mapper, &mut framealloc);

	let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
	unsafe
	{
		page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)
	};

	allocator::init_heap(&mut mapper, &mut framealloc)
		.expect("[ERROR] FAILED TO INITIALIZE HEAP");

	let heap_value = Box::new(41);
	println!("[INFO] HEAP_VALUE AT {:p}", heap_value);

	let mut vec = Vec::new();
	for i in 0..500
	{
		vec.push(i);
	}
	println!("[INFO] VEC AT {:p}", vec.as_slice());

	let refcounted = Rc::new(vec![1, 2, 3]);
	let clonedref = refcounted.clone();
	println!("[INFO] CURRENT REFERENCE COUNT: {}", Rc::strong_count(&clonedref));
	core::mem::drop(refcounted);
	println!("[INFO] CURRENT REFERENCE COUNT: {}", Rc::strong_count(&clonedref));

	#[cfg(test)]
	testexec();

	libertyos_kernel::hltloop();
}


// This is used in the event of a panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
	println!("{}", info);
	libertyos_kernel::hltloop();
}


/*
	TESTING
*/

// This is used in the event of a panic, when running tests.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
	libertyos_kernel::test_panic_handler(info)
}


// TEST CASE #1: TRIVASSERT
fn trivassert()
{
	assert_eq!(1, 1);
}
