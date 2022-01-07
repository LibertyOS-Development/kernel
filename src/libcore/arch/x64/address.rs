// src/arch/x64/address.rs
//
// Establishes core functionality of physical/virtual addresses (PhysicalAddress and VirtualAddress, respectively).

/*
	IMPORTS
*/

use bit_field::BitField;
use core::{fmt, ops::{Add, AddAssign, Sub, SubAssign}};


// 64-bit virtual memory address
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VirtualAddress(u64);


// 64-bit physical memory address
#[derive(Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PhysicalAddress(u64);
