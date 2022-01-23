// src/libcore/sys/sc/mod.rs
//
// Syscalls for LibertyOS.


/*
	IMPORTS
*/
use core::arch::asm;

use crate::{sc, libcore::{fs::directory_entry::FileInfo, sys::sc}};


// Services
pub mod svc;


/*
	CONSTANTS
*/

// Exit
pub const EXIT: usize = 0x1;

// Spawn
pub const SPAWN: usize = 0x2;

// Read
pub const READ: usize = 0x3;

// Write
pub const WRITE: usize = 0x4;

// Open
pub const OPEN: usize = 0x5;

// Close
pub const CLOSE: usize = 0x6;

// Info
pub const INFO: usize = 0x7;

// Duplicate
pub const DUPLICATE: usize = 0x8;

// Sleep
pub const SLEEP: usize = 0x9;

// Uptime
pub const UT: usize = 0xA;

// Real-time
pub const RT: usize = 0xB;

// Unknown system call
pub const UNKNOWN: usize = 0x26;



// Dispatcher for system-calls
pub fn dispatch(n: usize, a1: usize, a2: usize, a3: usize) -> usize
{
	match n
	{
		// Close
		CLOSE =>
		{
			let handle = a1;
			close(handle);
			0
		}


		// Duplicate
		DUPLICATE =>
		{
			let original = a1;
			let new = a2;
			crate::libcore::sys::sc::svc::dp(original, new) as usize
		}


		// Read
		READ =>
		{
			let handle = a1;
			let ptr = crate::libcore::sys::proc::ptr_from_address(a2 as u64);
			let len = a3;
			let buffer = unsafe
			{
				core::slice::from_raw_parts_mut(ptr, len)
			};

			crate::libcore::sys::sc::svc::rd(handle, buffer) as usize
		}

		// Real-time
		RT =>
		{
			// TODO: Convert time, using FL32's conv_to_bits
			crate::libcore::sys::sc::svc::rt() as usize
		}


		// Sleep
		SLEEP =>
		{
			crate::libcore::sys::sc::svc::sl(a1 as f64);
			0
		}



		// SPAWN
		SPAWN =>
		{
			let ptr = crate::libcore::sys::proc::ptr_from_address(a1 as u64);
			let len = a2;
			let path = unsafe
			{
				core::str::from_utf8_unchecked(core::slice::from_raw_parts(ptr, len))
			};

			spawn(path);
			0
		}


		// Up-time
		UT =>
		{
			// TODO: Convert time, using FL32's conv_to_bits
			crate::libcore::sys::sc::svc::ut as usize
		}


		_ =>
		{
			// For anything else
			UNKNOWN
		}
	}
}



// System calls

// Close
pub fn close(handle: usize)
{
	unsafe
	{
		sc!(CLOSE, handle as usize)
	};
}


// Duplicate
pub fn dup(original: usize, new: usize) -> Option<usize>
{
	let res = unsafe
	{
		sc!(DUPLICATE, original, new)
	} as isize;

	if res.is_negative()
	{
		None
	}
	else
	{
		Some(res as usize)
	}
}


// Exit
pub fn exit(code: usize) -> usize
{
	unsafe
	{
		sc!(EXIT, code as u64)
	}
}


// Info
pub fn info(path: &str) -> Option<FileInfo>
{
	let pathptr = path.as_ptr() as usize;
	let pathlen = path.len() as usize;
	let mut info = FileInfo::new();
	let infoptr = &mut info as *mut FileInfo as usize;
	let res = unsafe
	{
		sc!(INFO, pathptr, pathlen, infoptr)
	} as isize;

	if res.is_negative()
	{
		None
	}
	else
	{
		Some(info)
	}
}


// Open
pub fn open(path: &str, flags: usize) -> Option<usize>
{
	let ptr = path.as_ptr() as usize;
	let len = path.len() as usize;
	let res = unsafe
	{
		sc!(OPEN, ptr, len, flags)
	} as isize;

	if res.is_negative()
	{
		None
	}
	else
	{
		Some(res as usize)
	}
}


// Read
pub fn read(handle: usize, buffer: &mut [u8]) -> Option<usize>
{
	let ptr = buffer.as_ptr() as usize;
	let len = buffer.len() as usize;
	let res = unsafe
	{
		sc!(READ, handle, ptr, len)
	} as isize;

	if res.is_negative()
	{
		None
	}
	else
	{
		Some(res as usize)
	}
}


// Real-time
pub fn rt() -> f64
{
	let res = unsafe
	{
		sc!(RT)
	};
	f64::from_bits(res as u64)
}


// Sleep
pub fn sleep(sec: f64)
{
	unsafe
	{
		sc!(SLEEP, sec.to_bits())
	};
}


// Spawn
pub fn spawn(path: &str)
{
	let ptr = path.as_ptr() as usize;
	let len = path.len() as usize;

	unsafe
	{
		sc!(SPAWN, ptr, len)
	};
}


// Uptime
pub fn uptime() -> f64
{
	let res = unsafe
	{
		sc!(UT)
	};

	f64::from_bits(res as u64)
}


// Write
pub fn write(handle: usize, buffer: &[u8]) -> Option<usize>
{
	let ptr = buffer.as_ptr() as usize;
	let len = buffer.len() as usize;
	let res = unsafe
	{
		sc!(WRITE, handle, ptr, len)
	} as isize;

	if res.is_negative()
	{
		None
	}
	else
	{
		Some(res as usize)
	}
}


// Send system-calls


// Syscall 0
pub unsafe fn sc0(n: usize) -> usize
{
	let res: usize;
	asm!("int 0x80", in("rax") n, lateout("rax") res);
	res
}


// Syscall 1
pub unsafe fn sc1(n: usize, a1: usize) -> usize
{
	let res: usize;
	asm!("int 0x80", in("rax") n, in("rdi") a1, lateout("rax") res);
	res
}


// Syscall 2
pub unsafe fn sc2(n: usize, a1: usize, a2: usize) -> usize
{
	let res: usize;
	asm!("int 0x80", in("rax") n, in("rdi") a1, in("rsi") a2, lateout("rax") res);
	res
}


// Syscall 3
pub unsafe fn sc3(n: usize, a1: usize, a2: usize, a3: usize) -> usize
{
	let res: usize;
	asm!("int 0x80", in("rax") n, in("rdi") a1, in("rsi") a2, in("rdx") a3, lateout("rax") res);
	res
}
