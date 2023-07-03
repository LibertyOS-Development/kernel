#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(libertyos_kernel::testexec)]
#![reexport_test_harness_main = "testmain"]

use core::panic::PanicInfo;
use libertyos_kernel::ser::serprintln;

#[no_mangle]
pub extern "C" fn _start() -> !
{
	test_sqrt();
	libertyos_kernel::ser::serprintln!("[FAILURE]");
	exit_qemu(QemuExitCode::Failed);
	loop {}
}


//fn testexec(tests: &[&dyn Fn()])
//{
//	unimplemented!();
//}

#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
	libertyos_kernel::test_panic_handler(info)
}


use libertyos_kernel::println;
//#[test_case]
fn test_sqrt()
{
	println!("[TEST] CHECKING ABILITY TO CALCULATE SQUARE-ROOTS.");
	use libertyos_kernel::math::float::fl::FL32;
	pub const ERR_LIM: f32 = 0.05;

	pub const TEST_CASES: &[(f32, f32)] = &[
		(1.0, 1.0),
	];


	#[test]
	fn test_manager()
	{
		for &(x, expected) in TEST_CASES
		{
			let sqrt_x = F32(x).sqrt();
			let passing_err_rate = x * ERR_LIM;
			let actual_err_rate = sqrt_x - expected;

			assert!(
				actual_err_rate <= passing_err_rate,
				"[ERR] RESULT {} DEVIATES BEYOND ALLOWED MARGIN OF ERROR: {} vs {}",
				actual_err_rate,
				sqrt_x,
				expected
			);
		}
	}
}
