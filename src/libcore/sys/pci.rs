// src/libcore/sys/pci.rs
//
// Basic PCI functionality for the LibertyOS kernel.


/*
	IMPORTS
*/

use alloc::vec::Vec;
use bit_field::BitField;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::port::Port;

use crate::println;


// PCIDEV
lazy_static!
{
	pub static ref PCIDEV: Mutex<Vec<DevConfig>> = Mutex::new(Vec::new());
}


// ConfigReg struct
pub struct ConfigReg
{
	// Address
	address: u32,

	// Address port
	aport: Port<u32>,

	// Data port
	dport: Port<u32>,
}


// DevConfig struct
#[derive(Debug, Clone, Copy)]
pub struct DevConfig
{
	// Base addresses
	pub base_addresses: [u32; 6],

	// Bus
	pub bus: u8,

	// Class
	pub class: u8,

	// Command
	pub cmd: u16,

	// Device
	pub dev: u8,

	// Device ID
	pub did: u16,

	// Function
	pub func: u8,

	// Interrupt Line
	pub intr_ln: u8,

	// Interrupt Pin
	pub intr_pin: u8,

	// Prog
	pub prog: u8,

	// Rev
	pub rev: u8,

	// Status
	pub status: u16,

	// Subclass
	pub subclass: u8,

	// Vendor ID
	pub vid: u16,
}


// Implementation of the ConfigReg struct
impl ConfigReg
{
	// New
	pub fn new(bus: u8, dev: u8, func: u8, offset: u8) -> Self
	{
		Self
		{
			dport: Port::new(0xCDC),
			aport: Port::new(0xCF8),
			address: 0x8000_0000 | ((bus as u32) << 16) | ((dev as u32) << 11) | ((func as u32) << 8) | ((offset as u32) & 0xFC),
		}
	}


	// Read
	pub fn read(&mut self) -> u32
	{
		unsafe
		{
			self.aport.write(self.address);
			self.dport.read()
		}
	}


	// Write
	pub fn write(&mut self, data: u32)
	{
		unsafe
		{
			self.aport.write(self.address);
			self.dport.write(data);
		}
	}
}


// Implementation of the DevConfig struct
impl DevConfig
{
	// New
	pub fn new(bus: u8, dev: u8, func: u8) -> Self
	{
		// Vendor ID
		let vid = get_vid(bus, dev, func);

		// Device ID
		let did = get_did(bus, dev, func);

		// Register
		let mut reg = ConfigReg::new(bus, dev, func, 0x04);
		let data = reg.read();
		let cmd = data.get_bits(0..16) as u16;
		let status = data.get_bits(16..32) as u16;

		let mut reg = ConfigReg::new(bus, dev, func, 0x08);
		let data = reg.read();
		let rev = data.get_bits(0..8) as u8;
		let prog = data.get_bits(8..16) as u8;
		let subclass = data.get_bits(16..24) as u8;
		let class = data.get_bits(24..32) as u8;

		let mut reg = ConfigReg::new(bus, dev, func, 0x3C);
		let data = reg.read();
		let intr_ln = data.get_bits(0..8) as u8;
		let intr_pin = data.get_bits(8..16) as u8;

		let mut base_addresses: [u32; 6] = [0; 6];

		for i in 0..6
		{
			let offset = 0x10 + ((i as u8) << 2);
			let mut reg = ConfigReg::new(bus, dev, func, offset);
			base_addresses[i] = reg.read();
		}


		Self
		{
			base_addresses,
			bus,
			class,
			cmd,
			dev,
			did,
			func,
			intr_ln,
			intr_pin,
			prog,
			rev,
			status,
			subclass,
			vid,
		}
	}

	// Enable bus-mastering
	pub fn enable_busmast(&mut self)
	{
		let mut reg = ConfigReg::new(self.bus, self.dev, self.func, 0x04);
		let mut data = reg.read();
		data.set_bit(2, true);
		reg.write(data);
	}
}


// Check bus
pub fn checkbus(bus: u8)
{
	for dev in 0..32
	{
		checkdev(bus, dev);
	}
}


// Check device
pub fn checkdev(bus: u8, dev: u8)
{
	let func = 0;
	let vid = get_vid(bus, dev, func);

	// Check if device exists
	if vid == 0xFFFF
	{
		return;
	}

	new_dev(bus, dev, func);

	let htype = get_htype(bus, dev, func);

	if htype & 0x80 != 0
	{
		for func in 1..8
		{
			let vid = get_vid(bus, dev, func);

			if vid != 0xFFFF
			{
				new_dev(bus, dev, func);
			}
		}
	}
}


// Find device
pub fn find_dev(vid: u16, did: u16) -> Option<DevConfig>
{
	for &dev in PCIDEV.lock().iter()
	{
		if dev.vid == vid && dev.did == did
		{
			return Some(dev);
		}
	}
	None
}


// Get Device ID
pub fn get_did(bus: u8, dev: u8, func: u8) -> u16
{
	let mut reg = ConfigReg::new(bus, dev, func, 0x00);
	reg.read().get_bits(16..32) as u16
}


// Get header-type
pub fn get_htype(bus: u8, dev: u8, func: u8) -> u8
{
	let mut reg = ConfigReg::new(bus, dev, func, 0x0C);
	reg.read().get_bits(16..24) as u8
}


// Get Vendor ID
pub fn get_vid(bus: u8, dev: u8, func: u8) -> u16
{
	let mut reg = ConfigReg::new(bus, dev, func, 0x00);
	reg.read().get_bits(0..16) as u16
}


// Initialization
pub fn init()
{
	for bus in 0..256
	{
		checkbus(bus as u8);
	}

	let devs = PCIDEV.lock();
	for dev in devs.iter()
	{
		if dev.class == 0x01 && dev.subclass == 0x01
		{
			let mut reg = ConfigReg::new(dev.bus, dev.dev, dev.func, 0x08);
			let mut data = reg.read();
			let prog_offset = 8;

			if dev.prog.get_bit(0)
			{
				if dev.prog.get_bit(1)
				{
					data.set_bit(prog_offset, false);
					reg.write(data);
				}
			}

			if dev.prog.get_bit(2)
			{
				if dev.prog.get_bit(3)
				{
					data.set_bit(prog_offset + 2, false);
					reg.write(data);
				}
			}
		}
	}
}

// List
pub fn ls() -> Vec<DevConfig>
{
	PCIDEV.lock().clone()
}


// Create a new device
pub fn new_dev(bus: u8, dev: u8, func: u8)
{
	let config = DevConfig::new(bus, dev, func);
	PCIDEV.lock().push(config);
	println!("[INFO] PCI {:04X}:{:02X}:{:02X} [{:04X}:{:04X}]\n", bus, dev, func, config.vid, config.did);
}
