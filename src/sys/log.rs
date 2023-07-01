// src/libcore/sys/log.rs
//
// Logger for the LibertyOS kernel.

/*
	IMPORTS
*/

use alloc::collections::VecDeque;
use core::sync::atomic::{AtomicBool, Ordering};
pub use log::{debug, error, info, set_max_level, warn};
use spin::Mutex;

use crate::println;


pub static LOG: Mutex<Option<Logger>> = Mutex::new(None);


// Logger struct
pub struct Logger
{
	data: VecDeque<u8>,
	size: usize,
}


// SysLogger struct
struct SysLog
{
	logfunc: fn(&log::Record),
	pub init: AtomicBool,
}


// Implementation of the Logger struct
impl Logger
{
	// New
	pub fn new(size: usize) -> Logger
	{
		Logger
		{
			data: VecDeque::with_capacity(size),
			size
		}
	}


	// Read
	pub fn read(&self) -> (&[u8], &[u8])
	{
		self.data.as_slices()
	}


	// Write
	pub fn write(&mut self, buffer: &[u8])
	{
		for &b in buffer
		{
			while self.data.len() + 1 >= self.size
			{
				self.data.pop_front();
			}

			self.data.push_back(b);
		}
	}
}


// Implementation of the log::Log trait for the SysLog struct
impl log::Log for SysLog
{
	fn enabled(&self, _: &log::Metadata<'_>) -> bool
	{
		false
	}


	// Flush
	fn flush(&self) {}


	fn log(&self, rec: &log::Record<'_>)
	{
		(self.logfunc)(&rec)
	}
}


// Initialization
pub fn init()
{
	*LOG.lock() = Some(Logger::new(1024 * 1024));
}


// Initialize logger
pub fn initlog(func: fn(&log::Record))
{
	unsafe
	{
		match LOGGER.init.load(Ordering::SeqCst)
		{
			false =>
			{
				::log::set_max_level(::log::LevelFilter::Info);
					LOGGER.logfunc = func;

					match ::log::set_logger(&LOGGER)
					{
						Ok(_) => ::log::info!("[LOG] LOGGER SUCCESSFULLY INITIALIZED"),
						Err(e) => println!("[ERR] COULD NOT INITIALIZE LOGGER: {}", e),
					}

				LOGGER.init.store(true, Ordering::SeqCst);
			},

			true => ::log::info!("[INFO] FAILED TO REINIT LOGGER"),
		}
	}
}


// LOGGER
static mut LOGGER: SysLog = SysLog
{
	logfunc: |_| {},
	init: AtomicBool::new(false),
};
