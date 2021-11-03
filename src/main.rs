#![feature(custom_test_frameworks)]
#![test_runner(crate::testexec)]
#![reexport_test_harness_main = "testmain"]
#![no_main]
#![no_std]

use core::panic::PanicInfo;
//use libertyos_kernel::println;
mod vgabuff;
mod ser;


#[no_mangle]
pub extern "C" fn _start() -> !
{
//	println!("#	 	###");
//	println!("#		 #");
//	println!("#		 #");
//	println!("#		 #");
//	println!("#		 #");
//	println!("######	###");


	println!("If you can read this text, LibertyOS' kernel has been loaded successfully.");
	#[cfg(test)]
	testmain();
	loop {}
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
