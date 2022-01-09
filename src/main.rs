#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_efiapi)]
#![feature(custom_test_frameworks)]
#![test_runner(libertyos_kernel::testexec)]
#![reexport_test_harness_main = "testexec"]
#![allow(dead_code)]
#![allow(deprecated)]
#![allow(named_asm_labels)]
#![allow(unused_features)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

extern crate alloc;

use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use embedded_graphics::{image::Image, prelude::*};
use libertyos_kernel::{print, println, task::{Task, kbd, simpexec::SimpleExec, exec::Exec}, time::sleep};
use tinybmp::DynamicBmp;
use vga::{ colors::{ Color16, TextModeColor }, writers::{ Graphics640x480x16, GraphicsWriter, ScreenCharacter, TextWriter, Text80x25} };

entry_point!(kernel_main);

// KSIZE is set to about 2MB.
pub const KSIZE: usize = 2 << 20;

fn kernel_main(bootinfo: &'static BootInfo) -> !
{
	use libertyos_kernel::mem::{self, BootInfoFrameAllocator};
	use libertyos_kernel::libcore::allocator;
	use x86_64::{structures::paging::Page, VirtAddr};
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

	allocator::init_heap(&mut mapper, &mut framealloc)
		.expect("[ERROR] FAILED TO INITIALIZE HEAP");

	let heap_value = Box::new(41);
	let mut vec = (0..500).collect::<Vec<i32>>();
	let refcounted = Rc::new(vec![1, 2, 3]);
	let clonedref = refcounted.clone();
	core::mem::drop(refcounted);

	let mut executor = Exec::new();

	// Welcome message
	println!("LIBERTYOS v0.14.2");
	println!("");

	#[cfg(test)]
	testexec();

	executor.spawn(Task::new(kbd::print_keypresses()));
	executor.run();
}


async fn async_num() -> u32
{
	42
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
