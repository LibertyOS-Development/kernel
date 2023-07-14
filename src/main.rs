#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(type_ascription)]
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
use embedded_graphics::{image::{Image, ImageRaw}, pixelcolor::BinaryColor, prelude::*};
use embedded_time::{duration::*, rate::*};
use libertyos_kernel::{print, println, task::{Task, kbd, simpexec::SimpleExec, exec::Exec}, time::sleep};
use tinybmp::Bmp;
use vga::{ colors::{ Color16, TextModeColor }, writers::{ Graphics640x480x16, GraphicsWriter, ScreenCharacter, TextWriter, Text80x25, PrimitiveDrawing} };

entry_point!(kernel_main);

// KSIZE is set to about 2MB.
pub const KSIZE: usize = 2 << 20;
const KURIOS_VERSION: &str = env!("CARGO_PKG_VERSION");


fn kernel_main(bootinfo: &'static BootInfo) -> !
{
	use libertyos_kernel::user::shell::exec;


	libertyos_kernel::init::start(bootinfo);
	println!("KURIOS v{}", KURIOS_VERSION);
	print!("\x1b[?25h");
	println!();

	/*
	let text_mode = Text80x25::new();
	let tmcolor = TextModeColor::new(Color16::Yellow, Color16::Black);

	let graphics_mode = Graphics640x480x16::new();

	graphics_mode.set_mode();
	graphics_mode.clear_screen(Color16::Black);
	graphics_mode.draw_line((80, 60), (80, 420), Color16::White);
	graphics_mode.draw_line((80, 60), (540, 60), Color16::White);
	graphics_mode.draw_line((80, 420), (540, 420), Color16::White);
	graphics_mode.draw_line((540, 420), (540, 60), Color16::White);
	graphics_mode.draw_line((80, 90), (540, 90), Color16::White);
	*/

// While duration < 5 sec, display welcome screen. 

	loop
	{
		if let Some(cmd) = option_env!("LIBERTYOS_CMD")
		{
			let prompt = libertyos_kernel::user::shell::promptstr(true);
			println!("{}{}", prompt, cmd);
			libertyos_kernel::user::shell::exec(cmd);

//			libertyos_kernel::sys::acpi::shutdown;
		}
	}
}


#[rustfmt::skip]
const DATA: &[u8] = &[
	0b11101111, 0b0101_0000,
	0b10001000, 0b0101_0000,
	0b11101011, 0b0101_0000,
	0b10001001, 0b0101_0000,
	0b11101111, 0b0101_0000,
];

/*
fn display_logo<C>(data: &[u8], settings: &OutputSettings)
where
	C: PixelColor + From<Rgb555> + From<Rgb888> + Into<Rgb888>,
	{
		let bmp = Bmp::<C>::from_slice(&data).unwrap();
		let mut display = SimulatorDisplay::<Rgb888>::new(bmp.size());

		Image::new(&bmp, Point::zero())
			.draw(&mut display.color_converted())
			.unwrap();

		let mut window = Window::new(

*/

/*
fn start_shell()
{
	libertyos_kernel::user::shell::main(&["shell", script]).ok();
}
*/


/*
	libertyos_kernel::user::shell::main(&["shell"]);

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


//	let mut executor = Exec::new();
//	let prompt = libertyos_kernel::libcore::user::shell::promptstr(true);

	// Welcome message
	println!("LIBERTYOS v0.15.2");
	println!("");
	print!("\x1b[?25h");
//	print!("{}", prompt);


	#[cfg(test)]
	testexec();

//	executor.spawn(Task::new(kbd::print_keypresses()));
//	executor.run();

*/

pub fn shell()
{
	libertyos_kernel::user::shell::main(&["shell"]);
}


async fn async_num() -> u32
{
	42
}

// Handles allocation errors
pub fn alloc_err_handler(layout: alloc::alloc::Layout) -> !
{
	panic!("[ERR] ALLOCATION ERROR: {:?}", layout)
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
