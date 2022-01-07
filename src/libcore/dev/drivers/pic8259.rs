// dev/drivers/pic8259.rs
//
// This module provides support for the 8259 PIC.

use x86_64::instructions::port::Port;

// Sends the command to initialize the PIC.
const INIT_CMD: u8 = 0x11;
// Sends a command to acknowledge an interrupt has occurred.
const INTREND_CMD: u8 = 0x20;
// Sets the mode for the PIC to run in.
const MODE_8086: u8 = 0x01;


// A struct to handle individual PICs. This struct is not exported, as it is only accessed through another struct.
struct PIC
{
	// Base offset to use when mapping interrupts.
	offset: u8,
	// Set the I/O port that is used to send commands.
	cmd: Port<u8>,
	// Set the I/O port that is used to send and receive data.
	data: Port<u8>,
}

impl PIC
{
	// Checks if PIC is controlling the specific interrupt.
	// NOTE: Each PIC can handle eight (8) interrupts.
	fn handles(&self, intrid: u8) -> bool
	{
		self.offset <= intrid && intrid < self.offset + 8
	}

	// Sends a notification when interrupt has been handled, and the PIC can handle another interrupt.
	unsafe fn intrend(&mut self)
	{
		self.cmd.write(INTREND_CMD);
	}

	// Read the interrupt mask for the PIC.
	unsafe fn mask_read(&mut self) -> u8
	{
		self.data.read()
	}

	// Write the interrupt mask for the PIC.
	unsafe fn mask_write(&mut self, mask: u8)
	{
		self.data.write(mask)
	}
}


// This is a pair of PIC.
//
// Note: This type of interrupt-handling is standard on x86.
pub struct ChainPIC
{
	pics: [PIC; 2],
}

impl ChainPIC
{
	// Create an interface for the standard PIC1/PIC2, using offsets that are specified.
	pub const unsafe fn new(os1: u8, os2: u8) -> ChainPIC
	{
		ChainPIC
		{
			pics:
			[
				PIC
				{
					offset: os1,
					cmd: Port::new(0x20),
					data: Port::new(0x21),
				},
				PIC
				{
					offset: os2,
					cmd: Port::new(0xA0),
					data: Port::new(0xA1),
				},
			],
		}
	}

	// Initialize both PIC simultaneously.
	pub unsafe fn init(&mut self)
	{
		// Wait between writes to the PIC.
		let mut waitport: Port<u8>= Port::new(0x80);
		let mut wait = || waitport.write(0);

		// Save interrupt masks.
		let mask_save = self.mask_read();

		// Alert each PIC to an incoming initialization sequence.
		self.pics[0].cmd.write(INIT_CMD);
		wait();
		self.pics[1].cmd.write(INIT_CMD);
		wait();

		// Byte 1/3 - Establish base offsets.
		self.pics[0].data.write(self.pics[0].offset);
		wait();
		self.pics[1].data.write(self.pics[1].offset);
		wait();

		// Byte 2/3 - Coordinate the chain between the pair of PIC.
		self.pics[0].data.write(4);
		wait();
		self.pics[1].data.write(2);
		wait();

		// Byte 3/3 - Set the PIC mode.
		self.pics[0].data.write(MODE_8086);
		wait();
		self.pics[1].data.write(MODE_8086);
		wait();

		// Restore the previously saved masks.
		self.mask_write(mask_save[0], mask_save[1])
	}

	// Read interrupt mask for both PIC, respectively.
	pub unsafe fn mask_read(&mut self) -> [u8; 2]
	{
		[self.pics[0].mask_read(), self.pics[1].mask_read()]
	}

	// Write interrupt mask to each PIC, respectively.
	pub unsafe fn mask_write(&mut self, m1: u8, m2: u8)
	{
		self.pics[0].mask_write(m1);
		self.pics[1].mask_write(m2);
	}

	// Mask interrupts, disable both PIC.
	pub unsafe fn disable(&mut self)
	{
		self.mask_write(u8::MAX, u8::MAX)
	}

	// Checks if interrupt can be handled.
	pub fn handles(&self, intrid: u8) -> bool
	{
		self.pics.iter().any(|p| p.handles(intrid))
	}

	// Check if either PIC in the chain need to receive notification about the interrupt.
	pub unsafe fn notify_intrend(&mut self, intrid: u8)
	{
		if self.handles(intrid)
		{
			if self.pics[1].handles(intrid)
			{
				self.pics[1].intrend();
			}
			self.pics[0].intrend();
		}
	}
}
