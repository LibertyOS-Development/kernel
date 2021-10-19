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
