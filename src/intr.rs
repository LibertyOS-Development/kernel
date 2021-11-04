use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;


pub fn idtinit()
{
	let mut idt = InterruptDescriptorTable::new();
	idt.breakpoint.set_handler_fn(breakpnt_handler);
}

extern "x86-interrupt" fn breakpnt_handler(stackframe: InterruptStackFrame)
{
	println!("[EXC] BREAKPOINT\n{:#?}", stackframe);
}


