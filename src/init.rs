// src/init.rs
//
// This module handles initializing the various critical portions of the kernel. This module is called by the kernel upon startup.


/*
	IMPORTS
*/

use bootloader::BootInfo;

use crate::println;

// The start function, which is the main function of the init module
pub fn start(bootinfo: &'static BootInfo)
{
	// Initialize VGA
	println!("[INFO] INITIALIZING VGA");
	crate::libcore::graphics::vga::init();

	// Initialize GDT (global descriptor table)
	println!("[INFO] INITIALIZING GDT");
	crate::libcore::sys::gdt::init();

	// Initialize the IDT (interrupt descriptor table)
	println!("[INFO] INITIALIZING IDT");
	crate::libcore::sys::idt::init();

	// Initialize the PIC module, enable interrupts
	println!("[INFO] INITIALIZING PIC");
	crate::pic::init();

	// Initialize serial input/output
	println!("[INFO] INITIALIZING SER");
	crate::ser::init();

	// Initialize keyboard support
	println!("[INFO] INITIALIZING KEYBOARD SUPPORT");
	crate::libcore::dev::kbd::init();

	// Initialize time-keeping
	println!("[INFO] INITIALIZING TIME MANAGEMENT");
	crate::time::init();

	// Initialize basic memory management functions
	println!("[INFO] INITIALIZING MEMORY MANAGEMENT");
	crate::mem::init(bootinfo);

	// Initialize ATA support
	crate::libcore::fs::ata::init();

	// Initialize filesystem functions
	crate::libcore::fs::init();
}
