// src/libcore/sys/proc.rs
//
// Kernel processes.

/*
	IMPORTS
*/

#![allow(unused_mut)]

use alloc::{collections::BTreeMap, string::{String, ToString}};
use core::{arch::asm, sync::atomic::{AtomicU64, AtomicUsize, Ordering}};
use lazy_static::lazy_static;
use object::{Object, ObjectSegment};
use spin::RwLock;
use x86_64::{structures::idt::InterruptStackFrameValue, VirtAddr};

use crate::libcore::{sys::console::Console, sys::gdt::GDT, fs::{dev::Device, Resource}};


/*
	CONSTANTS
*/

// Code address
// NOTE: Set to 16 MB
pub static CODEADDRESS: AtomicU64 = AtomicU64::new((crate::libcore::allocator::HEAP_START as u64) + (16 << 20));

// Magic number for ELF executables
const ELFMAG: [u8; 4] = [0x74, b'E', b'L', b'F'];

// Maximum number of filehandles
const MAX_FILEHANDLE: usize = 16;

// Maximum number of processes
const MAX_PROC: usize = 2;

// Page size
pub const PAGESIZE: u64 = 4 * 1024;


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
	// Create
	pub fn create(bin: &[u8]) -> Result<usize, ()>
	{
		// Code size
		let code_size = 1024 * PAGESIZE;

		// Code address
		let code_address = CODEADDRESS.fetch_add(code_size, Ordering::SeqCst);

		// Allocate pages, using code address and code size
		crate::libcore::allocator::palloc(code_address, code_size);

		// Entry point
		let mut entrypt = 0;

		// Code pointer
		let cptr = code_address as *mut u8;

		// If binary is an ELF binary
		if bin[0..4] == ELFMAG
		{
			if let Ok(obj) = object::File::parse(bin)
			{
				entrypt = obj.entry();
				for seg in obj.segments()
				{
					let address = seg.address() as usize;
					if let Ok(data) = seg.data()
					{
						for (i, op) in data.iter().enumerate()
						{
							unsafe
							{
								let ptr = cptr.add(address + i);
								core::ptr::write(ptr, *op);
							}
						}
					}
				}
			}
		}
		else
		{
			// If binary is a raw binary
			for (i, op) in bin.iter().enumerate()
			{
				unsafe
				{
					let ptr = cptr.add(i);
					core::ptr::write(ptr, *op);
				}
			}
		}

		let mut tab = PROCTAB.write();
		let parent = &tab[id()];
		let data = parent.data.clone();
		let reg = parent.reg;
		let sf = parent.sf.clone();

		let id = MAXPID.fetch_add(1, Ordering::SeqCst);
		let proc = Proc
		{
			id,
			code_address,
			code_size,
			entrypt,
			data,
			sf,
			reg
		};

		Ok(id)
	}


	// Execute
	pub fn exec(&self)
	{
		setid(self.id);
		unsafe
		{
			asm!(
				"cli",
				"push rax",
				"push rsi",
				"push 0x200",
				"push rdx",
				"push rdi",
				"iretq",
				in("rax") GDT.1.userdata.0,
				in("rsi") self.code_address + self.code_size,
				in("rdx") GDT.1.usercode.0,
				in("rdi") self.code_address + self.entrypt,
			);
		}
	}

	// New
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


	// Spawn
	pub fn spawn(bin: &[u8])
	{
		if let Ok(pid) = Self::create(bin)
		{
			let proc =
			{
				let tab = PROCTAB.read();
				tab[pid].clone()
			};
			proc.exec();
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


// Spawn
pub fn spawn(path: &str) -> Result<(), ()>
{
	if crate::libcore::sys::sc::info(&path).is_some()
	{
		crate::libcore::sys::sc::spawn(&path);
		return Ok(());
	}
	Err(())
}


// User
pub fn user() -> String
{
	let tab = PROCTAB.read();
	let proc = &tab[id()];
	proc.data.directory.clone()
}
