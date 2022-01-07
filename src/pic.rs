// pic.rs

use crate::libcore::dev::drivers::pic8259::ChainPIC;
use spin::Mutex;

pub const PO1: u8 = 32;
pub const PO2: u8 = PO1 + 8;

pub static PICS: Mutex<ChainPIC> = Mutex::new(unsafe
{
	ChainPIC::new(PO1, PO2)
});

pub fn init()
{
	unsafe
	{
		PICS.lock().init();
	}
	x86_64::instructions::interrupts::enable();
}
