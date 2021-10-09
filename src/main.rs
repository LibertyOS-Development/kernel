#![no_main]
#![no_std]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
mod vgabuffer;
//extern crate goodnight;


entry_point!(kernel);

#[no_mangle]
fn kernel(bootinfo: &'static BootInfo) -> !
{
//	vgabuffer::print_whatever();
	loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
	loop {}
}
