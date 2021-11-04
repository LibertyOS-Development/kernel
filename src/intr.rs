use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use crate::println;


lazy_static!
{
	static ref IDT: InterruptDescriptorTable =
	{
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint.set_handler_fn(breakpnt_handler);
		idt
	};
}

pub fn idtinit()
{
	IDT.load();
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


#[test_case]
fn test_breakpnt_exc()
{
	x86_64::instructions::interrupts::int3();
}
