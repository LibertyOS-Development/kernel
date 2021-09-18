use core::panic::PanicInfo;

#[panic_handler]
fn pnc(_info: &PanicInfo) -> !
{
	loop {}
}

#![no_std]
fn main(){}
