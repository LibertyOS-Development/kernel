// time.rs
//
// Basic time-keeping functionality.


/*
	IMPORTS
*/

use core::hint::spin_loop;
use core::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use x86_64::instructions::{interrupts, port::Port};

use crate::cmos::CMOS;


// PID divider
const PITDIV: usize = 1193;

// PIT frequency
pub const PITFREQ: f64 = 3_579_545.0 / 3.0;

// PIT interval
const PITINTV: f64 = (PITDIV as f64) / PITFREQ;

// PIT tick
static PIT_TICK: AtomicUsize = AtomicUsize::new(0);

// Last RTC update
static LAST_RTCUPDATE: AtomicUsize = AtomicUsize::new(0);

// Clock per nanoseconds
static CLOCK_PER_NS: AtomicU64 = AtomicU64::new(0);



// Tick function
pub fn tick() -> usize
{
	PIT_TICK.load(Ordering::Relaxed)
}


// Time between ticks
pub fn time_between_ticks() -> f64
{
	PITINTV
}


// Last RTC update
pub fn last_rtcupdate() -> usize
{
	LAST_RTCUPDATE.load(Ordering::Relaxed)
}


// Halt
pub fn halt()
{
	let disabled = !interrupts::are_enabled();
	interrupts::enable_and_hlt();
	if disabled
	{
		interrupts::disable();
	}
}


// Initialization
pub fn init()
{
	// PIT
	let div = if PITDIV < 65536
	{
		PITDIV
	}
	else
	{
		0
	};

	let channel = 0;

	set_pitfreq_div(div as u16, channel);
	crate::libcore::sys::idt::set_irh(0, pit_intrh);


	// RTC
	crate::libcore::sys::idt::set_irh(8, rtc_intrh);
	crate::cmos::CMOS::new().enable_updateintr();


	// TSC

	// Set calibration time to around 0.25 seconds
	let calib = 250_000;
	let a = rdtsc();

	// Sleep for ~0.25 seconds
	sleep(calib as f64 / 1e6);

	let b = rdtsc();

	CLOCK_PER_NS.store((b - a) / calib, Ordering::Relaxed);
}



fn rdtsc() -> u64
{
	unsafe
	{
		core::arch::x86_64::_mm_lfence();
		core::arch::x86_64::_rdtsc()
	}
}


// Sleep
pub fn sleep(sec: f64)
{
	let start = crate::clock::uptime();

	while crate::clock::uptime() - start < sec
	{
		halt();
	}
}


// Wait (in nanoseconds)
pub fn nwait(nsec: u64)
{
	let start = rdtsc();
	let delta = nsec * CLOCK_PER_NS.load(Ordering::Relaxed);
	while rdtsc() - start < delta
	{
		spin_loop();
	}
}


// Set PIT frequency divider
pub fn set_pitfreq_div(divider: u16, channel: u8)
{
	interrupts::without_interrupts(||
	{
		let bytes = divider.to_le_bytes();
		let mut cmd: Port<u8> = Port::new(0x43);
		let mut data: Port<u8> = Port::new(0x40 + channel as u16);
		let opmode = 6;
		let accmode = 3;
		unsafe
		{
			cmd.write((channel << 6) | (accmode << 4) | opmode);
			data.write(bytes[0]);
			data.write(bytes[1]);
		}
	});
}


// PIT interrupt handler
pub fn pit_intrh()
{
	PIT_TICK.fetch_add(1, Ordering::Relaxed);
}


// RTC interrupt handler
pub fn rtc_intrh()
{
	LAST_RTCUPDATE.store(tick(), Ordering::Relaxed);
	CMOS::new().notify_intrend();
}
