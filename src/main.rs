#![no_main]
#![no_std]
use core::panic::PanicInfo;

#[panic_handler]
fn pnc(_info: &PanicInfo) -> !
{
	loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> !
{
	loop {}
}
