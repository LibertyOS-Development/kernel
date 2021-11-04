use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use lazy_static::lazy_static;

pub const DOUBLEFAULT_IST_IDX: u16 = 0;

lazy_static!
{
	static ref TSS: TaskStateSegment = {
		let mut tss = TaskStateSegment::new();
		tss.interrupt_stack_table[DOUBLEFAULT_IST_IDX as usize] = {
			const STACKSIZE: usize = 4096 * 5;
			static mut STACK: [u8; STACKSIZE] = [0; STACKSIZE];
			let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
			let stack_stop = stack_start + STACKSIZE;
			stack_stop
		};
		tss
	};
}
