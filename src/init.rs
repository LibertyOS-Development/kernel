// src/init.rs
//
// This module handles initializing the various critical portions of the kernel. This module is called by the kernel upon startup.


/*
	IMPORTS
*/

use bootloader::BootInfo;

use crate::{print, println};

// The start function, which is the main function of the init module
pub fn start(bootinfo: &'static BootInfo)
{
	/*	INITIALIZATION PROCESS

	LibertyOS has a simple initialization process. One by one, the
	critical functions (needed by the kernel) are initialized, with
	the order of initialization being dependent on required features
	for each component, respectively.

	Prior to initializing a component, a line is written to the screen,
	detailing the process.

	*/


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


	// Initialize logger
	println!("[INFO] INITIALIZING LOGGER");
	crate::libcore::sys::log::init();


	// Initialize CPU module
	println!("[INFO] INITIALIZING CPU MODULE");
	crate::libcore::sys::cpu::init();

	// Initialize PCI support
	println!("[INFO] INITIALIZING PCI SUPPORT");
	crate::libcore::sys::pci::init();

	// Initialize filesystem support
	println!("[INFO] INITIALIZING FILESYSTEM SUPPORT");
	crate::libcore::fs::init();

	// Initialize ATA support
	println!("[INFO] INITIALIZING ATA SUPPORT");
	crate::libcore::fs::ata::init();

/*
	// Create LibertyOS installation
	let csicolor = crate::libcore::sys::console::Style::color("Blue");
	let csireset = crate::libcore::sys::console::Style::reset();
	println!();

	print!("\nWould you like to create a new installation of LibertyOS? [Y/N]");
	if crate::libcore::io::stdin::Stdin.readln().trim() == "y"
	{
		println!("you entered y");
	}

*/
//	setup(true);
}


// Copy file
pub fn cp_file(pname: &str, buffer: &[u8], v: bool)
{
	if crate::libcore::fs::exists(pname)
	{
		return;
	}

	crate::libcore::fs::write(pname, buffer).ok();

	if v
	{
		println!("[INFO] COPIED FILE: {}", pname);
	}
}


// Create device
pub fn new_dev(pname: &str, dev: crate::libcore::fs::DevType, v: bool)
{
	if crate::libcore::sys::sc::info(pname).is_none()
	{
		if let Some(handle) = crate::libcore::fs::dev_new(pname, dev)
		{
			crate::libcore::sys::sc::close(handle);

			if v
			{
				println!("[INFO] NEW DEVICE: {}", pname);
			}
		}
	}
}


// Create directory
pub fn new_dir(pname: &str, v: bool)
{
	if let Some(handle) = crate::libcore::fs::directory_new(pname)
	{
		crate::libcore::sys::sc::close(handle);

		if v
		{
			println!("[INFO] NEW DIRECTORY: {}", pname);
		}
	}
}


// Set up a basic installation
pub fn setup(v: bool)
{
	new_dir("/bin", v);
	new_dir("/dev", v);
}
