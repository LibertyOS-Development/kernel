#![feature(custom_test_frameworks)]
#![test_runner(crate::testexec)]
#![reexport_test_harness_main = "testmain"]
#![no_main]
#![no_std]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
mod vgabuff;
mod ser;

entry_point!(kernmain);

#[no_mangle]
fn kernmain(bootinfo: &'static BootInfo) -> !
{
	use libertyos_kernel::mem::active_lvl4_tab;
	use x86_64::structures::paging::PageTable;
	use x86_64::VirtAddr;
	libertyos_kernel::init();

	#[cfg(test)]
	testmain();
	println!("LIBERTY-OS");
	println!("KERNEL VERSION 0.9.4");
	println!("");

	let physmem_offset = VirtAddr::new(bootinfo.physical_memory_offset);
	let l4tab = unsafe
	{
		active_lvl4_tab(physmem_offset)
	};
	for (i, entry) in l4tab.iter().enumerate()
	{
		if !entry.is_unused()
		{
			println!("[MSG] LVL4 ENTRY {}: {:?}", i, entry);

			let phys = entry.frame().unwrap().start_address();
			let virt = phys.as_u64() + bootinfo.physical_memory_offset;
			let ptr = VirtAddr::new(virt).as_mut_ptr();
			let l3tab: &PageTable = unsafe
			{
				&*ptr
			};

			for (i, entry) in l3tab.iter().enumerate()
			{
				if !entry.is_unused()
				{
					println!("[MSG] LVL3 ENTRY {}: {:?}", i, entry);
				}
			}
		}
	}
	#[cfg(test)]
	testmain();

	libertyos_kernel::hltloop();
}

#[cfg(not(test))] // PANIC HANDLER FOR RELEASE
#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
	println!("{}", info);
	libertyos_kernel::hltloop();
}

#[cfg(test)]
#[panic_handler] // PANIC HANDLER FOR DEBUG/TESTING
fn panic(info: &PanicInfo) -> !
{
	libertyos_kernel::test_panic_handler(info)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QEMUExitCode
{
	Success = 0x10,
	Failure = 0x11,
}

pub fn exitqemu(exitcode: QEMUExitCode)
{
	use x86_64::instructions::port::Port; //TODO: Use x64.
	unsafe
	{
		let mut port = Port::new(0xf4);
		port.write(exitcode as u32);
	}
}


pub trait CanTest
{
	fn run(&self) -> ();
}

impl<T> CanTest for T
where
	T: Fn(),
{
	fn run(&self)
	{
		serprint!("{}...\t", core::any::type_name::<T>());
		self();
		serprintln!("[SUCCESS]");
	}
}

#[cfg(test)]
fn testexec(tests: &[&dyn CanTest])
{
	serprintln!("[LIBERTYOS] EXECUTING {} TESTS", tests.len());
	for test in tests
	{
		test.run();
	}
	exitqemu(QEMUExitCode::Success);
}

#[test_case]
fn test_trivassert()
{
	assert_eq!(1, 1);
}
