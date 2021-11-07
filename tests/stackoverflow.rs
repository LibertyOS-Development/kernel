#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use libertyos_kernel::{exitqemu, QEMUExitCode, serprint, serprintln};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static!
{
	static ref TEST_IDT: InterruptDescriptorTable =
	{
		let mut idt = InterruptDescriptorTable::new();
		unsafe
		{
			idt.double_fault
				.set_handler_fn(test_double_fault_handler)
				.set_stack_index(libertyos_kernel::gdt::DOUBLEFAULT_IST_IDX);
		}
		idt
	};
}


pub fn init_test_idt()
{
	TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(_stackframe: InterruptStackFrame, _errcode: u64) -> !
{
	serprintln!("[SUCCESS]");
	exitqemu(QEMUExitCode::Success);
	loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> !
{
	serprint!("STACKOVERFLOW::STACK_OVERFLOW...\t");
	libertyos_kernel::gdt::init();
	init_test_idt();
	stackoverflow();
	panic!("[MSG] EXEC CONTINUED AFTER STACK OVERFLOW");
}

#[allow(unconditional_recursion)]
fn stackoverflow()
{
	stackoverflow();
	volatile::Volatile::new(0).read();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
	libertyos_kernel::test_panic_handler(info)
}
