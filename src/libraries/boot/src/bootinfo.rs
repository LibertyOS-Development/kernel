#![no_std]
use core::{ops, slice};

#[derive(Debug)]
#[repr(C)]
#[non_exhaustive]
pub struct BootInfo
{
	pub vers_maj: u16,
	pub vers_min: u16,
	pub vers_patch: u16,
	pub prerel: bool,
	pub memreg: MemReg,
	pub framebuffer: Optional<FrameBuffer>,
	pub physmem_offset: Optional<u64>,
	pub recursive_idx: Optional<u16>,
	pub rsdp_addr: Optional<u64>,
	pub tls_temp: Optional<TlsTemp>,
}

#[derive(Debug)]
#[repr(C)]
pub struct MemReg
{
	pub(crate) ptr: *mut MemReg,
	pub(crate) len: usize,
}

impl ops::Deref for MemReg
{
	type Tgt = [MemReg];
	fn deref(&self) -> &Self::Tgt
	{
		unsafe
		{
			slice::from_raw_parts(self.ptr, self.len)
		}
	}
}

impl ops::DerefMut for MemReg
{
	fn derefmut(&mut self) -> &mut Self::Tgt
	{
		unsafe
		{
			slice::from_raw_parts_mut(self.ptr, self.len(
		}
	}
}

impl From<&'static mut [MemReg]> for MemReg
{
	fn from(reg: &'static mut [MemReg]) -> Self
	{
		MemReg
		{
			ptr: reg.as_mut_ptr(),
			len: reg.len(),
		}
	}
}

impl From<MemReg> for &'static mut [MemReg]
{
	fn from(reg: MemReg) -> &'static mut [MemReg]
	{
		unsafe
		{
			slice::from_raw_parts_mut(reg.ptr, reg.len)
		}
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct MemReg
{
	pub start: u64,
	pub end: u64,
	pub kind: MemRegKind,
}

impl MemReg
{
	pub const fn empty() -> Self
	{
		MemReg
		{
			start: 0,
			end: 0,
			kind: MemRegKind::Bootloader,
		}
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
#[repr(C)]
pub enum MemRegKind
{
	Usable,
	Bootloader,
	UnknownUEFI(u32),
	UnknownBIOS(u32),
}

#[derive(Debug)]
#[repr(C)]
pub struct FrameBuffer
{
	pub(crate) start_buffer: u64,
	pub(crate) buffer_bytelen: usize,
	pub(crate) info: FrameBufferInfo,
}

impl FrameBuffer
{
	pub fn buffer(&self) -> &[u8]
	{
		unsafe
		{
			self.new_buffer()
		}
	}
	pub fn buffermut(&mut self) -> &mut [u8]
	{
		unsafe
		{
			self.new_buffer()
		}
	}
	unsafe fn new_buffer<'a>(&self) -> &'a mut [u8]
	{
		unsafe
		{
			slice::from_raw_parts_mut(self.buffer_start as *mut u8, self.buffer_bytelen)
		}
	}
	pub fn info(&self) -> FrameBufferInfo
	{
		self.info
	}
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FrameBufferInfo
{
	pub bytelen: usize,
	pub res_h: usize,
	pub res_v: usize,
	pub pxlfmt: PxlFmt,
	pub bytes_per_pxl: usize,
	pub stride: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[repr(C)]
pub enum PxlFmt
{
	RGB,
	BGR,
	U8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct TlsTemp
{
	pub start_addr: u64,
	pub filesize: u64,
	pub memsize: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum Optional<T>
{
	Some(T),
	None,
}

impl<T> Optional<T>
{
	pub fn into_opt(self) -> Option<T>
	{
		self.into()
	}
	pub const fn asref(&self) -> Option<&T>
	{
		match self
		{
			Self::Some(x) => Some(x),
			Self::None => None,
		}
	}
	pub fn asmut(&mut self) -> Option<&mut T>
	{
		match self
		{
			Self::Some(x) => Some(x),
			Self::None => None,
		}
	}
}

impl<T> From<Option<T>> for Optional<T>
{
	fn from(v: Option<T>) -> Self
	{
		match v
		{
			Some(v) => Optional::Some(v),
			None => Optional::None,
		}
	}
}

impl<T> From<Optional<T>> for Option<T>
{
	fn from(optional: Optional<T>) -> Option<T>
	{
		match optional
		{
			Optional::Some(v) => Some(v),
			Optional::None => None,
		}
	}
}

extern "C" fn _assert_ffi(_boot_info: BootInfo) {}
