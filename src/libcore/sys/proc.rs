// src/libcore/sys/proc.rs
//
// Kernel processes.

/*
	IMPORTS
*/

use alloc::{collections::BTreeMap, string::{String, ToString}};
use core::sync::atomic::{AtomicUsize, Ordering};
use lazy_static::lazy_static;
use spin::RwLock;
use x86_64::{structures::idt::InterruptStackFrameValue, VirtAddr};

use crate::libcore::{sys::console::Console, fs::{dev::Device, Resource}};


/*
	CONSTANTS
*/

// Magic number for ELF executables
const ELFMAG: [u8; 4] = [0x74, b'E', b'L', b'F'];

// Maximum number of filehandles
const MAX_FILEHANDLE: usize = 16;

// Maximum number of processes
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
	sf: InterruptStackFrameValue,
}


// ProcData Struct
#[derive(Clone, Debug)]
pub struct ProcData
{
	env: BTreeMap<String, String>,
	directory: String,
	user: Option<String>,
	filehandle: [Option<Resource>; MAX_FILEHANDLE],
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
			sf: isf,
			reg: Reg::default(),
			data: ProcData::new("/", None),
		}
	}
}


// Implementation of the ProcData struct
impl ProcData
{
	// New
	pub fn new(directory: &str, user: Option<&str>) -> Self
	{
		let env = BTreeMap::new();
		let directory = directory.to_string();
		let user = user.map(String::from);
		let mut filehandle = [(); MAX_FILEHANDLE].map(|_| None);

		filehandle[0] = Some(Resource::Device(Device::Console(Console::new())));
		filehandle[1] = Some(Resource::Device(Device::Console(Console::new())));
		filehandle[2] = Some(Resource::Device(Device::Console(Console::new())));

		Self
		{
			env,
			directory,
			user,
			filehandle
		}

	}
}


// Code address
pub fn ca() -> u64
{
	let tab = PROCTAB.read();
	let proc = &tab[id()];
	proc.code_address
}


// Directory
pub fn directory() -> String
{
	let tab = PROCTAB.read();
	let proc = &tab[id()];
	proc.data.directory.clone()
}


// Environment
pub fn env(key: &str) -> Option<String>
{
	let tab = PROCTAB.read();
	let proc = &tab[id()];
	proc.data.env.get(key).cloned()
}


// Environments
pub fn envs() -> BTreeMap<String, String>
{
	let tab = PROCTAB.read();
	let proc = &tab[id()];
	proc.data.env.clone()
}


// Exit
pub fn exit()
{
	let tab = PROCTAB.read();
	let proc = &tab[id()];
	crate::mem::p_dealloc(proc.code_address, proc.code_address);
	MAXPID.fetch_sub(1, Ordering::SeqCst);
	setid(0);
}


// File handle
pub fn fh(handle: usize) -> Option<Resource>
{
	let tab = PROCTAB.read();
	let proc = &tab[id()];
	proc.data.filehandle[handle].clone()
}


// Delete file handle
pub fn fh_del(handle: usize)
{
	let mut tab = PROCTAB.write();
	let proc = &mut tab[id()];
	proc.data.filehandle[handle] = None;
}


// Create a new file handle
pub fn fh_new(file: Resource) -> Result<usize, ()>
{
	let mut tab = PROCTAB.write();
	let proc = &mut tab[id()];

	let min = 4;
	let max = MAX_FILEHANDLE;
	for handle in min..max
	{
		if proc.data.filehandle[handle].is_none()
		{
			proc.data.filehandle[handle] = Some(file);
			return Ok(handle);
		}
	}
	Err(())
}


// Update file handle
pub fn fh_update(handle: usize, file: Resource)
{
	let mut tab = PROCTAB.write();
	let proc = &mut tab[id()];
	proc.data.filehandle[handle] = Some(file);
}


// ID
pub fn id() -> usize
{
	PID.load(Ordering::SeqCst)
}


// Pointer from address
pub fn ptr_from_address(address: u64) -> *mut u8
{
	(ca() + address) as *mut u8
}


// Registers
pub fn reg() -> Reg
{
	let tab = PROCTAB.read();
	let proc = &tab[id()];
	proc.reg
}


// Set code address
pub fn set_ca(address: u64)
{
	let mut tab = PROCTAB.write();
	let mut proc = &mut tab[id()];
	proc.code_address = address;
}


// Set directory
pub fn setdir(directory: &str)
{
	let mut tab = PROCTAB.write();
	let proc = &mut tab[id()];
	proc.data.directory = directory.into();
}


// Set environment
pub fn setenv(key: &str, val: &str)
{
	let mut tab = PROCTAB.write();
	let proc = &mut tab[id()];
	proc.data.env.insert(key.into(), val.into());
}


// Set ID
pub fn setid(id: usize)
{
	PID.store(id, Ordering::SeqCst)
}


// Set registers
pub fn setreg(reg: Reg)
{
	let mut tab = PROCTAB.write();
	let mut proc = &mut tab[id()];
	proc.reg = reg
}


// Set stack-frame
pub fn setsf(sf: InterruptStackFrameValue)
{
	let mut tab = PROCTAB.write();
	let mut proc = &mut tab[id()];
	proc.sf = sf;
}


// Set user
pub fn setuser(user: &str)
{
	let mut tab = PROCTAB.write();
	let proc = &mut tab[id()];
	proc.data.user = Some(user.into())
}


// Stack-frame
pub fn sf() -> InterruptStackFrameValue
{
	let tab = PROCTAB.read();
	let proc = &tab[id()];
	proc.sf.clone()
}


// User
pub fn user() -> String
{
	let tab = PROCTAB.read();
	let proc = &tab[id()];
	proc.data.directory.clone()
}
