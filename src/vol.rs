// src/vol.rs
//
// Volatile wrapper

/*
	IMPORTS
*/

use core::{fmt, intrinsics, marker::PhantomData, ops::{Deref, DerefMut, Index, IndexMut, Range, RangeBounds}, ptr, slice::{range, SliceIndex}};

// Helper traits for Read/Write:
pub trait CanRead {}
pub trait CanWrite {}

// Zero-sized type to allow read/write access:
#[derive(Debug, Copy, Clone)]
pub struct ReadWrite;
impl CanRead for ReadWrite {}
impl CanWrite for ReadWrite {}

// Zero-sized type to allow read access:
#[derive(Debug, Copy, Clone)]
pub struct Read;
impl CanRead for Read {}

// Zero-sized type to allow write access:
#[derive(Debug, Copy, Clone)]
pub struct Write;
impl CanWrite for Write {}


// Wrap reference, make referenced value volatile:
#[derive(Clone)]
#[repr(transparent)]
pub struct VOL<R, A = ReadWrite>
{
	access: PhantomData<A>,
	reference: R,
}

// Functions to use when creating new values:
impl<R> VOL<R>
{
	// Create new volatile instance, wrap specified reference:
	pub const fn new(reference: R) -> VOL<R>
	{
		VOL
		{
			access: PhantomData,
			reference,
		}
	}

	// Create new read-only volatile instance, wrap specified reference:
	pub const fn new_ro(reference: R) -> VOL<R, Read>
	{
		VOL
		{
			access: PhantomData,
			reference,
		}
	}

	// Create new write-only volatile instance, wrap specified reference:
	pub const fn new_wo(reference: R) -> VOL<R, Write>
	{
		VOL
		{
			access: PhantomData,
			reference,
		}
	}
}

impl<R, T, A> VOL<R, A>
where
	R: Deref<Target = T>,
	T: Copy,
{
	// Volatile read of contained value:
	pub fn read(&self) -> T
	where
		A: CanRead,
	{
		unsafe
		{
			ptr::read_volatile(&*self.reference)
		}
	}

	// Volatile write, sets value to specified value:
	pub fn write(&mut self, value: T)
	where
		A: CanWrite,
		R: DerefMut,
	{
		unsafe
		{
			ptr::write_volatile(&mut *self.reference, value)
		};
	}

	// Use specified closure and volatile instruction(s) to update contained value:
	pub fn update<F>(&mut self, f: F)
	where
		A: CanRead + CanWrite,
		R: DerefMut,
		F: FnOnce(&mut T),
	{
		let mut value = self.read();
		f(&mut value);
		self.write(value);
	}
}


// Method to extracted wrapped value:
impl<R, A> VOL<R, A>
{
	// Extract inner value:
	pub fn extract_inner(self) -> R
	{
		self.reference
	}
}


// Methods for transformations:
impl<R, T, A> VOL<R, A>
where
	R: Deref<Target = T>,
	T: ?Sized,
{
	// Map wrapped value to create a new VOL reference:
	pub fn map<'a, F, U>(&'a self, f: F) -> VOL<&'a U, A>
	where
		F: FnOnce(&'a T) -> &'a U,
		U: ?Sized,
		T: 'a,
	{
		VOL
		{
			access: self.access,
			reference: f(self.reference.deref()),
		}
	}

	// Map wrapped value to create a mutable VOL reference:
	pub fn mapmut<'a, F, U>(&'a mut self, f: F) -> VOL<&'a mut U, A>
	where
		F: FnOnce(&mut T) -> &mut U,
		R: DerefMut,
		U: ?Sized,
		T: 'a,
	{
		VOL
		{
			access: self.access,
			reference: f(&mut self.reference),
		}
	}
}


// Volatile-slices:
impl<T, R, A> VOL<R, A>
where
	R: Deref<Target = [T]>,
{
	// Apply index-operation to wrapped slice:
	pub fn idx<'a, I>(&'a self, idx: I) -> VOL<&'a I::Output, A>
	where
		I: SliceIndex<[T]>,
		T: 'a,
	{
		self.map(|slice| slice.index(idx))
	}

	// Apply mutable index-operation to wrapped slice:
	pub fn idxmut<'a, I>(&'a mut self, idx: I) -> VOL<&mut I::Output, A>
	where
		I: SliceIndex<[T]>,
		R: DerefMut,
		T: 'a,
	{
		self.mapmut(|slice| slice.index_mut(idx))
	}

	// Use volatile memory to copy elements from self into dst:
	pub fn cp_into_slice(&self, dst: &mut [T])
	where
		T: Copy,
	{
		assert_eq!(self.reference.len(), dst.len(), "DEST/SRC SLICES HAVE DIFFERING LENGTHS");
		unsafe
		{
			intrinsics::volatile_copy_nonoverlapping_memory(dst.as_mut_ptr(), self.reference.as_ptr(), self.reference.len());
		}
	}

	// Use volatile memory to copy elements from src into self:
	pub fn cp_from_slice(&mut self, src: &[T])
	where
		T: Copy,
		R: DerefMut,
	{
		assert_eq!(self.reference.len(), src.len(), "DEST/SRC SLICES HAVE DIFFERING LENGTHS");
		unsafe
		{
			intrinsics::volatile_copy_nonoverlapping_memory(self.reference.as_mut_ptr(), src.as_ptr(), self.reference.len());
		}
	}

	// Use volatile memmove to copy elements from one part of slice to another part:
	pub fn cpinternal(&mut self, src: impl RangeBounds<usize>, dest: usize)
	where
		T: Copy,
		R: DerefMut,
	{
		let Range
		{
			start: src_start,
			end: src_end,
		} = range(src, ..self.reference.len());
		let count = src_end - src_start;
		assert!(dest <= self.reference.len() - count, "DESTINATION OUT OF BOUNDS");

		unsafe
		{
			intrinsics::volatile_copy_memory(self.reference.as_mut_ptr().add(dest), self.reference.as_ptr().add(src_start), count);
		}
	}
}


// Volatile byte-slices
impl<R, A> VOL<R, A>
where
	R: Deref<Target = [u8]>,
{
	// Use volatile memset to set all elements in byte-slice to a specific value:
	pub fn fill(&mut self, value: u8)
	where
		R: DerefMut,
	{
		unsafe
		{
			intrinsics::volatile_set_memory(self.reference.as_mut_ptr(), value, self.reference.len());
		}
	}
}


// Array to slice conversion
impl<R, A, T, const N: usize> VOL<R, A>
where
	R: Deref<Target = [T; N]>,
{
	// Convert array reference to shared slice:
	pub fn as_slice(&self) -> VOL<&[T], A>
	{
		self.map(|array| &array[..])
	}

	// Convert mutable array reference to mutable slice:
	pub fn as_mut_slice(&mut self) -> VOL<&mut [T], A>
	where
		R: DerefMut,
	{
		self.mapmut(|array| &mut array[..])
	}
}

// Restrictions
impl<R> VOL<R>
{
	// Restrict to read-only:
	pub fn ro(self) -> VOL<R, Read>
	{
		VOL
		{
			access: PhantomData,
			reference: self.reference,
		}
	}

	// Restrict to write-only:
	pub fn wo(self) -> VOL<R, Write>
	{
		VOL
		{
			access: PhantomData,
			reference: self.reference,
		}
	}
}


// Debuggery and whatnot
impl<R, T, A> fmt::Debug for VOL<R, A>
where
	R: Deref<Target = T>,
	T: Copy + fmt::Debug,
	A: CanRead,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		f.debug_tuple("VOL").field(&self.read()).finish()
	}
}

impl<R> fmt::Debug for VOL<R, Write>
where
	R: Deref,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		f.debug_tuple("VOL").field(&"[WO]").finish()
	}
}
