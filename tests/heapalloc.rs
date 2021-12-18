#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(libertyos_kernel::testexec)]
#![reexport_test_harness_main = "testexec"]

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use libertyos_kernel::allocator::HEAP_SIZE;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> !
{
	use libertyos_kernel::allocator;
	use libertyos_kernel::mem::{self, BootInfoFrameAllocator};
	use x86_64::VirtAddr;

	libertyos_kernel::init();
	let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
	let mut mapper = unsafe
	{
		mem::init(physical_memory_offset)
	};

	let mut frame_allocator = unsafe
	{
		BootInfoFrameAllocator::init(&boot_info.memory_map)
	};
	allocator::init_heap(&mut mapper, &mut frame_allocator).expect("[ERR] FAILED TO INITIALIZE HEAP");

	testexec();
	loop {}
}


#[test_case]
fn simple_alloc()
{
	let heapval1 = Box::new(41);
	let heapval2 = Box::new(13);
	assert_eq!(*heapval1, 41);
	assert_eq!(*heapval2, 13);
}

#[test_case]
fn largevec()
{
	let n = 1000;
	let mut vec = Vec::new();
	for i in 0..n
	{
		vec.push(i);
	}
	assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn manyboxes()
{
	for i in 0..HEAP_SIZE
	{
		let x = Box::new(i);
		assert_eq!(*x, i);
	}
}


// This is the panic handler.
#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
	libertyos_kernel::test_panic_handler(info)
}
