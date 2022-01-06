// src/arch/x64/instruct/mod.rs
//
// The mod.rs file for the arch::x64::instruct module.

/*
	IMPORTS
*/
use core::arch::asm;


// Halt CPU until next interrupt
#[inline]
pub fn cpu_halt()
{
	unsafe
	{
		asm!("hlt", options(nomem, nostack, preserves_flags));
	}
}

// Execute "nop", which does...nothing (no, really). This is used to stop the CPU from performing any operations.
#[inline]
pub fn cpu_nop()
{
	unsafe
	{
		asm!("nop", options(nomem, nostack, preserves_flags));
	}
}


// Breakpoint for Bochs
pub fn bochs_breakpoint()
{
	unsafe
	{
		asm!("xchg bx, bx", options(nomem, nostack, preserves_flags));
	}
}

