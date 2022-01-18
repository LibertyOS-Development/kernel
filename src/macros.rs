// macros.rs
// This file is responsible for all the macros used by the kernel.

/*
	IMPORTS
*/

use crate::libcore::fs::directory_read;


// block
#[macro_export]
macro_rules! block
{
	($e:expr) =>
	{
		loop
		{
			#[allow(unreachable_patterns)]
			match $e
			{
				Err($crate::noblkio::Err::Other(e)) =>
				{
					#[allow(unreachable_code)]
					break Err(e)
				}
				Err($crate::noblkio::Err::WouldBlk) => {}
				Ok(x) => break Ok(x),
			}
		}
	};
}


// impl_display_measurement
//
// This macro is used to implement fmt::Display for measurements.
#[macro_export]
macro_rules! impl_display_measurement
{
	($($t:ty)*) => ($(
		impl core::fmt::Display for $t
		{
			fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result
			{
				let (unit, value) = self.crate::math::measurements::measurement::ret_apt_units();
				// Value:
				value.fmt(f)?;
				write!(f, "\u{00A0}{}", unit)
			}
		}
	)*)
}


/*
// irh
//
// Handler for interrupt-requests
#[macro_export]
macro_rules! irh
{
	($handler:ident, $it:expr) =>
	{
		pub extern "x86-interrupt" fn $handler(_stack_frame: InterruptStackFrame)
		{
			let handlers = IR_HANDLERS.lock();
			handlers[$ir]();
			unsafe
			{
				crate::pic::PICS.lock().notify_intrend(interrupt_index($ir));
			}
		}
	};
}
*/


// print
//
// This macro is analagous to the "print!" macro (std), except for the fact that this macro prints to the VGA text buffer.
#[macro_export]
macro_rules! print
{
	($($arg:tt)*) => ($crate::libcore::sys::console::printfmt(format_args!($($arg)*)));
}


// println
//
// This macro is analagous to the "println!" macro (std), except for the fact that this macro prints to the VGA text buffer.
#[macro_export]
macro_rules! println
{
	() => ($crate::print!("\n"));
	($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}


// read_ui_func
//
// Read an unsigned integer function
#[macro_export]
macro_rules! read_ui_func
{
	($name:ident, $type:ident) =>
	{
		fn $name(&mut self) -> $type
		{
			let data = self.blk.data();
			let a = self.blk_data_offset;
			let b = a + core::mem::size_of::<$type>();
			self.blk_data_offset = b;
			$type::from_be_bytes(data[a..b].try_into().unwrap())
		}
	};
}


// sc
//
// This macro is used to make system-calls.
#[macro_export]
macro_rules! sc
{
	($n:expr) => (
		$crate::libcore::sys::sc::sc0($n as usize));

	($n:expr, $a1:expr) => (
		$crate::libcore::sys::sc::sc1($n as usize, $a1 as usize));

	($n:expr, $a1:expr, $a2:expr) => (
		$crate::libcore::sys::sc::sc2($n as usize, $a1 as usize, $a2 as usize));

	($n:expr, $a1:expr, $a2:expr, $a3:expr) => (
		$crate::libcore::sys::sc::sc3($n as usize, $a1 as usize, $a2 as usize, $a3 as usize));
}


// waitfor
#[macro_export]
macro_rules! waitfor
{
	($cond:expr) =>
	{
		while !$cond
		{
			core::hint::spin_loop()
		}
	};
}
