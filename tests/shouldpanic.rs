#![no_std]
#![no_main]

use core::panic::PanicInfo;
use libertyos_kernel::{exitqemu, serprint, serprintln, QEMUExitCode};

#[no_mangle]
pub extern "C" fn _start() -> !
{
	shouldfail();
	serprintln!("[ERR] TEST DID NOT PANIC");
	exitqemu(QEMUExitCode::Failure);
	loop {}
}

fn shouldfail()
{
	serprint!("SHOULDPANIC::SHOULDFAIL...\t");
	assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
	serprintln!("[SUCCESS]");
	exitqemu(QEMUExitCode::Success);
	loop {}
}
