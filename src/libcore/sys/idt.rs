// src/libcore/sys/idt.rs
//
// Interrupt descriptor table.


/*
	IMPORTS
*/

use core::arch::asm;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::{structures::idt::{InterruptDescriptorTable, InterruptStackFrame, InterruptStackFrameValue, PageFaultErrorCode}, instructions::{interrupts, port::Port}};

use crate::{print, println, libcore::sys::proc::Reg};


// P1 constant
pub const P1: u16 = 0x21;

// P2 constant
pub const P2: u16 = 0xA1;


// Lazy static wrapper around the IR_HANDLES reference
lazy_static!
{
	// Establishes the IR_HANDLERS reference, which is based on the def_iq_handler function
	pub static ref IR_HANDLERS: Mutex<[fn(); 16]> = Mutex::new([def_ir_handler; 16]);


	// The IDT static reference
	static ref IDT: InterruptDescriptorTable =
	{
		// Create a new IDT
		let mut idt = InterruptDescriptorTable::new();

		// Set the IDT's breakpoint-handler to be the breakpoint-handler that the kernel already has
		idt.breakpoint.set_handler_fn(breakpoint_handler);

		unsafe
		{
			// Double-faults
			idt.double_fault.
				set_handler_fn(double_fault_handler).
				set_stack_index(crate::libcore::sys::gdt::DOUBLEFAULT_IST_IDX);

			// Page-faults
			idt.page_fault.
				set_handler_fn(page_fault_handler).
				set_stack_index(crate::libcore::sys::gdt::PAGE_FAULT_ISTIDX);

			// General protection faults
			idt.general_protection_fault.
				set_handler_fn(gen_prot_fault_handler).
				set_stack_index(crate::libcore::sys::gdt::GEN_PROT_FAULT_ISTIDX);

			// [0x80]
			idt[0x80].
				set_handler_fn(core::mem::transmute(wrapped_sch as *mut fn())).
				set_stack_index(crate::libcore::sys::gdt::DOUBLEFAULT_IST_IDX).
				set_privilege_level(x86_64::PrivilegeLevel::Ring3);
		}


		// Interrupt index: 0
		idt[intridx(0) as usize].set_handler_fn(ir0h);

		// Interrupt index: 1
		idt[intridx(1) as usize].set_handler_fn(ir1h);

		// Interrupt index: 2
		idt[intridx(2) as usize].set_handler_fn(ir2h);

		// Interrupt index: 3
		idt[intridx(3) as usize].set_handler_fn(ir3h);

		// Interrupt index: 4
		idt[intridx(4) as usize].set_handler_fn(ir4h);

		// Interrupt index: 5
		idt[intridx(5) as usize].set_handler_fn(ir5h);

		// Interrupt index: 6
		idt[intridx(6) as usize].set_handler_fn(ir6h);

		// Interrupt index: 7
		idt[intridx(7) as usize].set_handler_fn(ir7h);

		// Interrupt index: 8
		idt[intridx(8) as usize].set_handler_fn(ir8h);

		// Interrupt index: 9
		idt[intridx(9) as usize].set_handler_fn(ir9h);

		// Interrupt index: 10
		idt[intridx(10) as usize].set_handler_fn(ir10h);

		// Interrupt index: 11
		idt[intridx(11) as usize].set_handler_fn(ir11h);

		// Interrupt index: 12
		idt[intridx(12) as usize].set_handler_fn(ir12h);

		// Interrupt index: 13
		idt[intridx(13) as usize].set_handler_fn(ir13h);

		// Interrupt index: 14
		idt[intridx(14) as usize].set_handler_fn(ir14h);

		// Interrupt index: 15
		idt[intridx(15) as usize].set_handler_fn(ir15h);

		// Set stack-segment fault-handler
		idt.stack_segment_fault.set_handler_fn(stack_segment_fault_handler);

		// Set handler for no segment found
		idt.segment_not_present.set_handler_fn(seg_not_found_handler);

		// Return the resulting version of the IDT
		idt
	};
}

// Default interrupt-request handler
pub fn def_ir_handler() {}


// Initialize
pub fn init()
{
	// Load the IDT
	IDT.load();
}


// The intridx function, which translates interrupt-requests into system-interrupts
pub fn intridx(ir: u8) -> u8
{
	crate::pic::PO1 + ir
}


/*
	HANDLERS
*/

// Breakpoint
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame)
{
	print!("[ERR] BREAKPOINT EXCEPTION\n{:#?}\n", stack_frame);
}


// Double fault
extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
	panic!("[ERR] DOUBLE FAULT: \n{:#?}", stack_frame);
}


// General protection fault
extern "x86-interrupt" fn gen_prot_fault_handler(stack_frame: InterruptStackFrame, _err_code: u64)
{
	panic!("[ERR] GENERAL PROTECTION FAULT: \n{:#?}", stack_frame);
}


// Page fault
extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode)
{
	let ip = stack_frame.instruction_pointer.as_ptr();
	let inst: [u8; 8] = unsafe
	{
		core::ptr::read(ip)
	};

	println!("[INFO] CODE: {:?}", inst);
	panic!("[ERR] PAGE FAULT: \n{:#?}\n{:#?}", stack_frame, error_code);
}


// Segment not found
extern "x86-interrupt" fn seg_not_found_handler(stack_frame: InterruptStackFrame, _error_code: u64)
{
	panic!("[ERR] SEGMENT NOT FOUND\n{:#?}", stack_frame);
}


// Stack segment fault
extern "x86-interrupt" fn stack_segment_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64)
{
	panic!("[ERR] STACK SEGMENT FAULT: \n{:#?}", stack_frame);
}


// Interrupt-request handler macro
macro_rules! irh
{
	($handler: ident, $ir:expr) =>
	{
		pub extern "x86-interrupt" fn $handler(_stack_frame: InterruptStackFrame)
		{
			let handlers = IR_HANDLERS.lock();
			handlers[$ir]();

			unsafe
			{
				crate::pic::PICS.lock().notify_intrend(intridx($ir));
			}
		}
	};
}


// Interrupt-request handlers

// Interrupt index: 0
irh!(ir0h, 0);

// Interrupt index: 1
irh!(ir1h, 1);

// Interrupt index: 2
irh!(ir2h, 2);

// Interrupt index: 3
irh!(ir3h, 3);

// Interrupt index: 4
irh!(ir4h, 4);

// Interrupt index: 5
irh!(ir5h, 5);

// Interrupt index: 6
irh!(ir6h, 6);

// Interrupt index: 7
irh!(ir7h, 7);

// Interrupt index: 8
irh!(ir8h, 8);

// Interrupt index: 9
irh!(ir9h, 9);

// Interrupt index: 10
irh!(ir10h, 10);

// Interrupt index: 11
irh!(ir11h, 11);

// Interrupt index: 12
irh!(ir12h, 12);

// Interrupt index: 13
irh!(ir13h, 13);

// Interrupt index: 14
irh!(ir14h, 14);

// Interrupt index: 15
irh!(ir15h, 15);



// Wrap macro
macro_rules! wrap
{
	($fn: ident => $w:ident) =>
	{
//		#[naked]
		pub unsafe extern "sysv64" fn $w()
		{
			asm!(
				"push rax",
				"push rcx",
				"push rdx",
				"push rsi",
				"push rdi",
				"push r8",
				"push r9",
				"push r10",
				"push r11",
				"mov rsi, rsp",
				"mov rdi, rsp",
				"add rdi, 9 * 8",
				"call {}",
				"pop r11",
				"pop r10",
				"pop r9",
				"pop r8",
				"pop rdi",
				"pop rsi",
				"pop rdx",
				"pop rcx",
				"pop rax",
				"iretq",
				sym $fn,
				options(noreturn)
			);
		}
	}
}


// Wrap sch, convert into wrapped_sch
wrap!(sch => wrapped_sch);


// sch
extern "sysv64" fn sch(stack_frame: &mut InterruptStackFrame, reg: &mut Reg)
{
	let n = reg.rax;
	let a1 = reg.rdi;
	let a2 = reg.rsi;
	let a3 = reg.rdx;

	// Create backup of the CPU context
	if n == crate::libcore::sys::sc::SPAWN
	{
		crate::libcore::sys::proc::setsf(stack_frame.clone());
		crate::libcore::sys::proc::setreg(*reg);
	}

	let res = crate::libcore::sys::sc::dispatch(n, a1, a2, a3);


	// Restore from backup
	if n == crate::libcore::sys::sc::EXIT
	{
		let stackframe = crate::libcore::sys::proc::sf();

		unsafe
		{
			core::ptr::write_volatile(stack_frame.as_mut().extract_inner() as *mut InterruptStackFrameValue, stackframe);
			core::ptr::write_volatile(reg, crate::libcore::sys::proc::reg());
		}
	}

	reg.rax = res;

	unsafe
	{
		crate::pic::PICS.lock().notify_intrend(0x80)
	};
}


// Clear interrupt-request mask
pub fn clr_irmask(ir: u8)
{
	let mut port: Port<u8> = Port::new(if ir < 8
	{
		P1
	}
	else
	{
		P2
	});

	unsafe
	{
		let val = port.read() & !(1 << if ir < 8
		{
			ir
		}
		else
		{
			ir - 8
		});

		port.write(val);
	}
}


// Set interrupt-request handler
pub fn set_irh(ir: u8, handler: fn())
{
	interrupts::without_interrupts(||
	{
		let mut handlers = IR_HANDLERS.lock();
		handlers[ir as usize] = handler;

		// Clear interrupt-request mask
		clr_irmask(ir);
	});
}


// Set interrupt-request mask
pub fn set_irmask(ir: u8)
{
	let mut port: Port<u8> = Port::new(if ir < 8
	{
		P1
	}
	else
	{
		P2
	});

	unsafe
	{
		let val = port.read() | (1 << (if ir < 8
		{
			ir
		}
		else
		{
			ir - 8
		}));

		port.write(val);
	}
}
