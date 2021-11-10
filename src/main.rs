#![feature(custom_test_frameworks)]
#![test_runner(crate::testexec)]
#![reexport_test_harness_main = "testmain"]
#![no_main]
#![no_std]

use core::panic::PanicInfo;
//use embedded_graphics::{image::Image, prelude::*, pixelcolor::Rgb565};
//use time::*;
//use tinybmp::Bmp;
mod vgabuff;
mod ser;


#[no_mangle]
pub extern "C" fn _start() -> !
{
	libertyos_kernel::init();

	#[cfg(test)]
	testmain();
	println!("LIBERTY-OS");
	println!("KERNEL VERSION 0.6.0");
	loop
	{
		use libertyos_kernel::print;
		print!("-");
	}

//	let libertyoslogo = include_bytes!("graphics/images/bmp/Logo-Light.bmp");
//	let logo = Bmp::<Rgb565>::from_slice(libertyoslogo).unwrap();
//	Image::new(&logo, Point::new(10, 20)).draw(&mut vgabuff::Buffer)?;
}

#[cfg(not(test))] // PANIC HANDLER FOR RELEASE
#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
	println!("{}", info);
	loop {}
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
