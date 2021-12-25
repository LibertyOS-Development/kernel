// macros.rs
// This file is responsible for all the macros used by the kernel.


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
				Err($crate::noblio::Err::Other(e)) =>
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


// print
//
// This macro is analagous to the "print!" macro (std), except for the fact that this macro prints to the VGA text buffer.
#[macro_export]
macro_rules! print
{
	($($arg:tt)*) => ($crate::vgabuff::_print(format_args!($($arg)*)));
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
