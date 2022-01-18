
#![no_std]
#![cfg_attr(test, no_main)]
#![allow(dead_code)]
#![allow(named_asm_labels)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(asm_sym)]
#![feature(const_mut_refs)]
#![feature(core_intrinsics)]
#![feature(custom_test_frameworks)]
#![feature(naked_functions)]
#![feature(slice_range)]
#![feature(type_ascription)]
#![test_runner(crate::testexec)]
#![reexport_test_harness_main = "testmain"]


extern crate alloc;
extern crate core;

use core::{ops::Deref, panic::PanicInfo};

pub mod libcore;
pub mod clock;
pub mod cmos;
pub mod ctypes;
pub mod font;
pub mod init;
pub mod intr;
pub mod macros;
pub mod mem;
pub mod noblkio;
pub mod pic;
pub mod rgx;
pub mod ser;
pub mod time;
pub mod vol;

// This is set to be 2MB.
pub const KSIZE: usize = 2 << 20;

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> !
{
	panic!("[ERR] ALLOC ERR: {:?}", layout)
}

pub fn init()
{
	libcore::sys::gdt::init();
	intr::idtinit();
	unsafe
	{
		intr::PICS.lock().init()
	};
	x86_64::instructions::interrupts::enable();
}

pub fn hltloop() -> !
{
	loop
	{
		x86_64::instructions::hlt();
	}
}

// This implementation is used for the FAT filesystem.
pub trait BlockDevice
{
	type Error;
	fn read(&self, buf: &mut [u8], address: usize, numblk: usize) -> Result<(), Self::Error>;
	fn write(&self, buf: &[u8], address: usize, numblk: usize) -> Result<(), Self::Error>;
}

pub trait CanTest
{
	fn exec(&self) -> ();
}

impl<T> CanTest for T
where
	T: Fn(),
{
	fn exec(&self)
	{
		serprint!("{}...\t", core::any::type_name::<T>());
		self();
		serprintln!("[SUCCESS]");
	}
}

pub fn testexec(tests: &[&dyn CanTest])
{
	serprintln!("RUNNING {} TESTS:", tests.len());
	for test in tests
	{
		test.exec();
	}
	exitqemu(QEMUExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> !
{
	serprintln!("[FAILURE]\n");
	serprintln!("[ERR]: {}\n", info);
	exitqemu(QEMUExitCode::Failure);
	hltloop();
}

// This section provides the kernel wth the StableDeref trait.

pub unsafe trait StableDeref: Deref {}
pub unsafe trait CloneStableDeref: StableDeref + Clone {}

use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

use core::cell::{Ref, RefMut};

unsafe impl<T: ?Sized> StableDeref for Box<T> {}
unsafe impl<T> StableDeref for Vec<T> {}
unsafe impl StableDeref for String {}
unsafe impl<'a> StableDeref for Cow<'a, str> {}
unsafe impl<'a, T: Clone> StableDeref for Cow<'a, [T]> {}
unsafe impl<T: ?Sized> StableDeref for Rc<T> {}
unsafe impl<T: ?Sized> CloneStableDeref for Rc<T> {}
unsafe impl<T: ?Sized> StableDeref for Arc<T> {}
unsafe impl<T: ?Sized> CloneStableDeref for Arc<T> {}
unsafe impl<'a, T: ?Sized> StableDeref for Ref<'a, T> {}
unsafe impl<'a, T: ?Sized> StableDeref for RefMut<'a, T> {}
unsafe impl<'a, T: ?Sized> StableDeref for &'a T {}
unsafe impl<'a, T: ?Sized> CloneStableDeref for &'a T {}
unsafe impl<'a, T: ?Sized> StableDeref for &'a mut T {}

// This section of lib.rs provides the kernel with the AsSlice and AsMutSlice
// traits of the standard library, minus the standard library.

pub trait AsSlice
{
	type Elem;
	fn as_slice(&self) -> &[Self::Elem];
}

pub trait AsMutSlice: AsSlice
{
	fn asmutslice(&mut self) -> &mut [Self::Elem];
}

impl<'a, S> AsSlice for &'a S
where
	S: ?Sized + AsSlice,
{
	type Elem = S::Elem;
	fn as_slice(&self) -> &[S::Elem]
	{
		(**self).as_slice()
	}
}

impl<'a, S> AsSlice for &'a mut S
where
	S: ?Sized + AsSlice,
{
	type Elem = S::Elem;
	fn as_slice(&self) -> &[S::Elem]
	{
		(**self).as_slice()
	}
}

impl<'a, S> AsMutSlice for &'a mut S
where
	S: ?Sized + AsMutSlice,
{
	fn asmutslice(&mut self) -> &mut [S::Elem]
	{
		(**self).asmutslice()
	}
}

impl<T> AsSlice for [T]
{
	type Elem = T;
	fn as_slice(&self) -> &[T]
	{
		self
	}
}

impl<T> AsMutSlice for [T]
{
	fn asmutslice(&mut self) -> &mut [T]
	{
		self
	}
}

impl<T, const N: usize> AsSlice for [T; N]
{
	type Elem = T;
	fn as_slice(&self) -> &[T]
	{
		self
	}
}

impl<T, const N: usize> AsMutSlice for [T; N]
{
	fn asmutslice(&mut self) -> &mut [T]
	{
		self
	}
}


// TESTING


#[cfg(test)]
use bootloader::{BootInfo, entry_point};

#[cfg(test)]
//entry_point!(test_kernmain);

#[cfg(test)]
fn test_kernmain(_bootinfo: &'static BootInfo) -> !
{
	init();
	testmain();
	hltloop();
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> !
{
	init();
	testmain();
	hltloop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
	test_panic_handler(info)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QEMUExitCode
{
	Success = 0x10,
	Failure = 0x11,
}

pub fn exitqemu(exitcode: QEMUExitCode)
{
	use x86_64::instructions::port::Port;
	unsafe
	{
		let mut port = Port::new(0xf4);
		port.write(exitcode as u32);
	}
}
