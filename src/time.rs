// time.rs

use core::hint::spin_loop;
use core::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use x86_64::instructions::{interrupts, port::Port};

const PITDIV: usize = 1193;
pub const PITFREQ: f64 = 3_579_545.0 / 3.0;
const PITINTV: f64 = (PITDIV as f64) / PITFREQ;

static PIT_TICK: AtomicUsize = AtomicUsize::new(0);
static LAST_RTCUPDATE: AtomicUsize = AtomicUsize::new(0);
static CLOCK_PER_NS: AtomicU64 = AtomicU64::new(0);

pub fn tick() -> usize
{
	PIT_TICK.load(Ordering::Relaxed)
}

pub fn time_between_ticks() -> f64
{
	PITINTV
}

pub fn last_rtcupdate() -> usize
{
	LAST_RTCUPDATE.load(Ordering::Relaxed)
}

pub fn halt()
{
	let disabled = !interrupts::are_enabled();
	interrupts::enable_and_hlt();
	if disabled
	{
		interrupts::disable();
	}
}

fn rdtsc() -> u64
{
	unsafe
	{
		core::arch::x86_64::_mm_lfence();
		core::arch::x86_64::_rdtsc()
	}
}

//pub fn sleep(sec: f64)
//{
//	let start = 
