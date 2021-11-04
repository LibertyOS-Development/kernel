#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::testexec)]
#![reexport_test_harness_main = "testmain"]

use core::panic::PanicInfo;

pub mod intr;
pub mod ser;
pub mod vgabuff;
//pub mod tests;

pub trait CanTest
{
	fn exec(&self) -> ();
}

impl<T> CanTest for T
where
	T: Fn(),
{
	fn exec(&self)
	{
		serprint!("{}...\t", core::any::type_name::<T>());
		self();
		serprintln!("[SUCCESS]");
	}
}

pub fn testexec(tests: &[&dyn CanTest])
{
	serprintln!("RUNNING {} TESTS:", tests.len());
	for test in tests
	{
		test.exec();
	}
	exitqemu(QEMUExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> !
{
	serprintln!("[FAILURE]\n");
	serprintln!("[ERR]: {}\n", info);
	exitqemu(QEMUExitCode::Failure);
	loop {}
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> !
{
	testmain();
	loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
	test_panic_handler(info)
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
	use x86_64::instructions::port::Port;
	unsafe
	{
		let mut port = Port::new(0xf4);
		port.write(exitcode as u32);
	}
}
