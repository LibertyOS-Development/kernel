#![no_main]
#![no_std]

use core::panic::PanicInfo;
mod vgabuff;
//extern crate goodnight;


#[no_mangle]
pub extern "C" fn _start() -> !
{
	println!("WELCOME TO LIBERTYOS");
	loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
	println!("{}", info);
	loop {}
}
