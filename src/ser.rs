use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static!
{
	pub static ref SER1: Mutex<SerialPort> = {
		let mut serport = unsafe
		{
			SerialPort::new(0x3F8)
		};
		serport.init();
		Mutex::new(serport)
	};
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments)
{
	use core::fmt::Write;
		SER1.lock().write_fmt(args).expect("[ERR] FAILED TO PRINT TO SERIAL");
}

#[macro_export]
macro_rules! serprint {
	($($arg:tt)*) => {
		$crate::ser::_print(format_args!($($arg)*));		
	};
}

#[macro_export]
macro_rules! serprintln
{
	() => ($crate::serprint!("\n"));
	($fmt:expr) => ($crate::serprint!(concat!($fmt, "\n")));
	($fmt:expr, $($arg:tt)*) => ($crate::serprint!(concat!($fmt, "\n"), $($arg)*));
}