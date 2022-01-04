// src/arch/x64/reg/rflags.rs
//
// Store the processor state in RFLAGS register.

/*
	IMPORTS
*/
use bitflags::bitflags;

bitflags!
{
	// RFLAGS register:
	pub struct RFlags: u64
	{
		// CPU feature ID flag
		// NOTE: This flag can be modified if CPU supports CPUID.
		const ID = 1 << 24;

		// Pending external, maskable interrupt
		// NOTE: This is used if CR4.VME/CR4.PVI have been activated.
		const VIRT_INTR_PENDING = 1 << 20;

		// Virtual image of INTRFLAG bit
		// NOTE: This is used if CR4.VME/CR4.PVI have been activated.
		const VIRT_INTR = 1 << 19;

		// Enable automatic alignment checking
		// NOTE: Requires CRO.AM to be set, CPL must equal 3.
		const ALIGN_CHECK = 1 << 18;

		// Enable virtual-8086 mode
		const VIRT_8086_MODE = 1 << 17;

		// Allow restarting instruction after instruction breakpoint
		const RES_FLAG = 1 << 16;

		// Used (by iret) to determine whether or not current task is nested, during hardware task switch mode
		const NEST_TASK = 1 << 14;

		// The high/maximum bit for I/O privilege level field, specifies required privlege level to execute I/O address-space instructions
		const IOPL_MAX = 1 << 13;

		// The lowest/minimum bit for I/O privilege level field, specifies required privilege level to execute I/O address-space instructions
		const IOPL_MIM = 1 << 12;

		// Set by hardware, in the event of the sign bit of the source operands differ from that of the result of the last signed integer
		const OVERFLOW_FLAG = 1 << 11;

		// Set the order in which strings are processed
		const DIRECTION_FLAG = 1 << 10;

		// Enable interrupts
		const INTR_FLAG = 1 << 9;

		// Enable single-step mode (for debugging)
		const TRAP_FLAG = 1 << 8;

		// Set by hardware, in the event of the last mathematical operation resulting in a negative value
		const SIGN_FLAG = 1 << 7;

		// Set by hardware, in the event of the last mathematical operation returning a value of zero
		const ZERO_FLAG = 1 << 6;

		// TODO; Add explanation
		const AUX_CARRY_FLAG = 1 << 4;

		// Set by hardware, in the event of the last result has an even number of single bits
		// NOTE: Only used in some operations
		const PARITY_FLAG = 1 << 2;

		// Set by hardware, in the event of the previous mathematical operation generating a carry-out of the most significant bit of the result
		const CARRY_FLAG = 1;
	}
}
