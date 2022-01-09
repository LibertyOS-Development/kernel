// src/libcore/sys/proc.rs
//
// Kernel processes.

/*
	IMPORTS
*/

use core::sync::atomic::{AtomicUsize, Ordering};
use lazy_static::lazy_static;
use spin::RwLock;
use x86_64::{structures::idt::InterruptStackFrameValue, VirtAddr};


/*
	CONSTANTS
*/

const MAX_FILEHANDLE: usize = 16;
const MAX_PROC: usize = 2;


lazy_static!
{
	pub static ref PID: AtomicUsize = AtomicUsize::new(0);
	pub static ref MAXPID: AtomicUsize = AtomicUsize::new(1);
	pub static ref PROCTAB: RwLock<[Proc; MAX_PROC]> = RwLock::new([(); MAX_PROC].map(|_| Proc::new(0)));
}


// Proc struct
#[derive(Clone, Debug)]
pub struct Proc
{
	code_address: u64,
	code_size: u64,
	data: ProcData,
	entrypt: u64,
	id: usize,
	reg: Reg,
	stackframe: InterruptStackFrameValue,
}


// Reg struct
#[derive(Debug, Clone, Copy, Default)]
pub struct Reg
{
	pub r11: usize,
	pub r10: usize,
	pub r9: usize,
	pub r8: usize,
	pub rdi: usize,
	pub rsi: usize,
	pub rdx: usize,
	pub rcx: usize,
	pub rax: usize,
}


// Implementation of the Proc struct
impl Proc
{
	pub fn new(id: usize) -> Self
	{
		let isf = InterruptStackFrameValue
		{
			code_segment: 0,
			cpu_flags: 0,
			instruction_pointer: VirtAddr::new(0),
			stack_pointer: VirtAddr::new(0),
			stack_segment: 0,
		};

		Self
		{
			id,
			code_address: 0,
			code_size: 0,
			entrypt: 0,
			stackframe: isf,
			reg: Reg::default(),
			data: ProcData::new("/", None),
		}
	}
}
