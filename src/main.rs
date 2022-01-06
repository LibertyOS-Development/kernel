#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(custom_test_frameworks)]
#![test_runner(libertyos_kernel::testexec)]
#![reexport_test_harness_main = "testexec"]
#![allow(dead_code)]
#![allow(deprecated)]
#![allow(unused_features)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

extern crate alloc;

use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use embedded_graphics::{image::Image, prelude::*};
use libertyos_kernel::println;
use tinybmp::DynamicBmp;
use vga::{ colors::{ Color16, TextModeColor }, writers::{ Graphics640x480x16, GraphicsWriter, ScreenCharacter, TextWriter, Text80x25} };

entry_point!(kernel_main);

// KSIZE is set to about 2MB.
pub const KSIZE: usize = 2 << 20;

fn kernel_main(bootinfo: &'static BootInfo) -> !
{
	use libertyos_kernel::mem::{self, BootInfoFrameAllocator};
	use libertyos_kernel::allocator;
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

	let bmpdat = include_bytes!("../resources/images/bmp/Logo-Dark.bmp");

	// This controls LibertyOS' text mode.
	let textmode = Text80x25::new();
	let tmcolor = TextModeColor::new(Color16::Yellow, Color16::Black);
//	let screenchar = ScreenCharacter::new("", tmcolor);


	// This creates LibertyOS' TUI.
	let graphicsmode = Graphics640x480x16::new();
	graphicsmode.set_mode();
	graphicsmode.clear_screen(Color16::Black);
	graphicsmode.draw_line((80, 60), (80, 420), Color16::White);
	graphicsmode.draw_line((80, 60), (540, 60), Color16::White);
	graphicsmode.draw_line((80, 420), (540, 420), Color16::White);
	graphicsmode.draw_line((540, 420), (540, 60), Color16::White);
	graphicsmode.draw_line((80, 90), (540, 90), Color16::White);

	for (offset, character) in "LibertyOS v0.13.11".chars().enumerate()
	{
		graphicsmode.draw_character(250 + offset * 8, 72, character, Color16::Red)
	}

	for (offset, character) in "Welcome to LibertyOS.".chars().enumerate()
	{
		graphicsmode.draw_character(100 + offset * 8, 100, character, Color16::Yellow)
	}

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
