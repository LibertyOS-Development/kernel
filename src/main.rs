#![no_main]
#![no_std]

use core::panic::PanicInfo;
mod vgabuff;
//extern crate goodnight;


#[no_mangle]
pub extern "C" fn _start() -> !
{
	loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
	loop {}
}
