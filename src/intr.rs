#![allow(dead_code)]

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use crate::{gdt, print, println};
use pic8259::ChainedPics;
use spin;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static!
{
	static ref IDT: InterruptDescriptorTable =
	{
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint.set_handler_fn(breakpnt_handler);
		idt[IntrIdx::Keyboard.asusize()].set_handler_fn(keyboard_interrupt_handler);
		unsafe
		{
			idt.double_fault
				.set_handler_fn(doubleflt_handler)
				.set_stack_index(gdt::DOUBLEFAULT_IST_IDX);
		}
		idt[IntrIdx::Timer.asusize()].set_handler_fn(timer_interrupt_handler);

		idt
	};
}

pub fn idtinit()
{
	IDT.load();
}


#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum IntrIdx
{
	Timer = PIC_1_OFFSET,
	Keyboard,
}

impl IntrIdx
{
	fn asu8(self) -> u8
	{
		self as u8
	}
	fn asusize(self) -> usize
	{
		usize::from(self.asu8())
	}
}



// Handles breakpoint execptions.
extern "x86-interrupt" fn breakpnt_handler(stackframe: InterruptStackFrame)
{
	println!("[EXC] BREAKPOINT\n{:#?}", stackframe);
}


// Handles double-fault exceptions.
extern "x86-interrupt" fn doubleflt_handler(stackframe: InterruptStackFrame, _errcode: u64) -> !
{
	panic!("[EXC] DOUBLE-FAULT\n{:#?}", stackframe);
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stackframe: InterruptStackFrame) {
	use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
	use spin::Mutex;
	use x86_64::instructions::port::Port;

	lazy_static!
	{
		static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
		Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
		);
	}
	let mut keyboard = KEYBOARD.lock();
	let mut port = Port::new(0x60);
	let scancode: u8 = unsafe { port.read() };
	if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
		if let Some(key) = keyboard.process_keyevent(key_event) {
			match key
			{
				DecodedKey::Unicode(character) => print!("{}", character),
				DecodedKey::RawKey(key) => print!("{:?}", key),
			}
		}
	}
	unsafe {
	PICS.lock().notify_end_of_interrupt(IntrIdx::Keyboard.asu8());
	}
}

use x86_64::structures::idt::PageFaultErrorCode;
use crate::hltloop;

extern "x86-interrupt" fn page_fault_handler(stackframe: InterruptStackFrame, errcode: PageFaultErrorCode)
{
	use x86_64::registers::control::Cr2;
	println!("[EXC] PAGE FAULT");
	println!("[ERR] ACCESSED ADDR: {:?}", Cr2::read());
	println!("[ERR] ERRCODE: {:?}", errcode);
	println!("{:#?}", stackframe);
	hltloop();
}


extern "x86-interrupt" fn timer_interrupt_handler(_stackframe: InterruptStackFrame)
{
//	print!(".");
	unsafe
	{
		PICS.lock().notify_end_of_interrupt(IntrIdx::Timer.asu8());
	}
}

#[test_case]
fn test_breakpnt_exc()
{
	x86_64::instructions::interrupts::int3();
}
