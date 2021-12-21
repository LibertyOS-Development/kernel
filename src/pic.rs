// pic.rs

use pic8259::ChainedPics;
use spin::Mutex;

pub const PO1: u8 = 32;
pub const PO2: u8 = PO1 + 8;

pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe
{
	ChainedPics::new(PO1, PO2)
});

pub fn init()
{
	unsafe
	{
		PICS.lock().initialize();
	}
	x86_64::instructions::interrupts::enable();
}
