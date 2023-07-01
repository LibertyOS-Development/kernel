// src/libcore/fs/ata.rs
//
// Used to work with ATA devices.

/*
	IMPORTS
*/

use alloc::{string::String, vec::Vec};
use bit_field::BitField;
use core::{convert::TryInto, fmt, hint::spin_loop};
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::port::{Port, PortReadOnly, PortWriteOnly};

use crate::{println, serprint, serprintln};


pub const BLKSIZE: usize = 512;


#[repr(u16)]
#[derive(Debug, Clone, Copy)]
enum Command
{
	ID = 0xEC,
	Read = 0x20,
	Write = 0x30,
}


enum IDResponse
{
	ATA([u16; 256]),
	ATAPI,
	SATA,
	None,
}


#[repr(usize)]
#[derive(Debug, Clone, Copy)]
enum Status
{
	// Error
	ERR = 0,

	// Index
	IDX = 1,

	// CORR
	CORR = 2,

	// Data request
	DRQ = 3,

	// DSC
	DSC = 4,

	// DF
	DF = 5,

	// Device ready
	DEVREADY = 6,

	// Busy
	BUSY = 7,
}


#[derive(Debug, Clone)]
pub struct Bus
{
	id: u8,
	irq: u8,

	alt_status_reg: PortReadOnly<u8>,
	cmd_reg: PortWriteOnly<u8>,
	ctl_reg: PortWriteOnly<u8>,
	data_reg: Port<u16>,
	drive_blkess_reg: PortReadOnly<u8>,
	drive_reg: Port<u8>,
	err_reg: PortReadOnly<u8>,
	features_reg: PortWriteOnly<u8>,
	sectcount_reg: Port<u8>,
	status_reg: PortReadOnly<u8>,

	lba0_reg: Port<u8>,
	lba1_reg: Port<u8>,
	lba2_reg: Port<u8>,
}


impl Bus
{
	// New
	pub fn new(id: u8, iobase: u16, ctlbase: u16, irq: u8) -> Self
	{
		Self
		{
			id, irq,

			alt_status_reg: PortReadOnly::new(ctlbase + 0),
			cmd_reg: PortWriteOnly::new(iobase + 7),
			ctl_reg: PortWriteOnly::new(ctlbase + 0),
			data_reg: Port::new(iobase + 0),
			drive_reg: Port::new(iobase + 7),
			drive_blkess_reg: PortReadOnly::new(ctlbase + 1),
			err_reg: PortReadOnly::new(iobase + 1),
			features_reg: PortWriteOnly::new(iobase + 1),
			sectcount_reg: Port::new(iobase + 2),
			status_reg: PortReadOnly::new(iobase + 7),

			lba0_reg: Port::new(iobase + 3),
			lba1_reg: Port::new(iobase + 4),
			lba2_reg: Port::new(iobase + 5),
		}
	}

	// CHECK FLOATING BUS
	fn check_floating_bus(&mut self) -> Result<(), ()>
	{
		match self.status()
		{
			0xFF | 0x7F => Err(()),
			_ => Ok(()),
		}
	}


	// CLEAR INTERRUPT
	fn clrintr(&mut self) -> u8
	{
		unsafe
		{
			self.status_reg.read()
		}
	}


	// DATA READ
	fn data_read(&mut self) -> u16
	{
		unsafe
		{
			self.data_reg.read()
		}
	}


	// DATA WRITE
	fn data_write(&mut self, data: u16)
	{
		unsafe
		{
			self.data_reg.write(data)
		}
	}


	// DEBUG
	fn debug(&mut self)
	{
		unsafe
		{
			serprintln!("[INFO] ATA STATUS_REG: 0b{:08b} <BUSY|DRIVEREADY|#|#|DRQ|#|#|ERR>", self.alt_status_reg.read());
			serprintln!("[INFO] ATA ERR_REG: 0b{:08b} <#|#|#|#|#|ABORT|#|#>", self.err_reg.read());
		}
	}


	// SELECT DRIVE
	fn drivesel(&mut self, drive: u8) -> Result<(), ()>
	{
		self.poll(Status::BUSY, false)?;
		self.poll(Status::DRQ, false)?;

		unsafe
		{
			self.drive_reg.write(0xA0 | (drive << 4))
		}

		// Wait for 400 nanoseconds
		crate::time::nwait(400);
		self.poll(Status::BUSY, false)?;
		self.poll(Status::DRQ, false)?;

		Ok(())
	}


	// ERROR
	fn error(&mut self) -> bool
	{
			self.status().get_bit(Status::ERR as usize)
	}


	// IDENTIFY DRIVE
	fn id_drive(&mut self, drive: u8) -> Result<IDResponse, ()>
	{
		if self.check_floating_bus().is_err()
		{
			return Ok(IDResponse::None);
		}

		self.drivesel(drive)?;
		self.write_cmd_params(drive, 0)?;

		if self.writecmd(Command::ID).is_err()
		{
			if self.status() == 0
			{
				return Ok(IDResponse::None);
			}
			else
			{
				return Err(());
			}
		}

		match (self.lba1(), self.lba2())
		{
			(0x00, 0x00) => Ok(IDResponse::ATA([(); 256].map(|_|
			{
				self.data_read()
			}))),

			(0x14, 0xEB) => Ok(IDResponse::ATAPI),
			(0x3C, 0x3C) => Ok(IDResponse::SATA),
			(_, _) => Err(()),
			}
		}



	// LBA1
	fn lba1(&mut self) -> u8
	{
		unsafe
		{
			self.lba1_reg.read()
		}
	}


	// LBA2
	fn lba2(&mut self) -> u8
	{
		unsafe
		{
			self.lba2_reg.read()
		}
	}


	// POLL
	fn poll(&mut self, bit: Status, val: bool) -> Result<(), ()>
	{
		let start = crate::clock::uptime();
		while self.status().get_bit(bit as usize) != val
		{
			if crate::clock::uptime() - start > 1.0
			{
				serprintln!("[INFO] ATA HUNG DURING POLLING OF {:?} BIT OF STATUS_REG", bit);
				self.debug();
				return Err(());
			}
			spin_loop();
		}
		Ok(())
	}


	// READ
	fn read(&mut self, drive: u8, blk: u32, buffer: &mut [u8]) -> Result<(), ()>
	{
	//	serprint!("{}", buffer.len() == BLKSIZE);
		self.setup_pio(drive, blk)?;
		self.writecmd(Command::Read)?;
		for chunk in buffer.chunks_mut(2)
		{
			let data = self.data_read().to_le_bytes();
			chunk.clone_from_slice(&data);
		}

		if self.error()
		{
			serprintln!("[ERR] DATA READ ERROR");
			self.debug();
			Err(())
		}
		else
		{
			Ok(())
		}
	}


	// RESET
	fn reset(&mut self)
	{
		unsafe
		{
			// SET SRST
			self.ctl_reg.write(4);

			// Wait for 5 nanoseconds
			self.wait(5);

			// Clear ctl_reg
			self.ctl_reg.write(0);

			// Wait for 2000 nanoseconds (2 milliseconds)
			self.wait(2000);
		}
	}


	// SETUP PIO
	fn setup_pio(&mut self, drive: u8, blk: u32) -> Result<(), ()>
	{
		self.drivesel(drive)?;
		self.write_cmd_params(drive, blk)?;
		Ok(())
	}

	// STATUS
	fn status(&mut self) -> u8
	{
		unsafe
		{
			self.alt_status_reg.read()
		}
	}


	// WAIT
	fn wait(&mut self, nsec: u64)
	{
		crate::time::nwait(nsec);
	}


	// WRITE
	fn write(&mut self, drive: u8, blk: u32, buffer: &[u8]) -> Result<(), ()>
	{
		serprint!("{}", buffer.len() == BLKSIZE);
		self.setup_pio(drive, blk)?;
		self.writecmd(Command::Write)?;
		for chunk in buffer.chunks(2)
		{
			let data = u16::from_le_bytes(chunk.try_into().unwrap());
			self.data_write(data);
		}

		if self.error()
		{
			serprintln!("[ERR] DATA WRITE ERROR");
			self.debug();
			Err(())
		}
		else
		{
			Ok(())
		}
	}

	// WRITE COMMAND
	fn writecmd(&mut self, cmd: Command) -> Result<(), ()>
	{
		unsafe
		{
			self.cmd_reg.write(cmd as u8)
		}

		// Wait for 400 nanoseconds
		self.wait(400);

		// Ignore first read
		self.status();
		self.clrintr();

		// If drive does not exist
		if self.status() == 0
		{
			return Err(());
		}

		self.poll(Status::BUSY, false)?;
		self.poll(Status::DRQ, true)?;

		Ok(())
	}


	// WRITE COMMAND PARAMETERS
	fn write_cmd_params(&mut self, drive: u8, blk: u32) -> Result<(), ()>
	{
		let lba = true;
		let mut bytes = blk.to_le_bytes();

		bytes[3].set_bit(4, drive > 0);
		bytes[3].set_bit(5, true);
		bytes[3].set_bit(6, lba);
		bytes[3].set_bit(7, true);

		unsafe
		{
			self.sectcount_reg.write(1);
			self.lba0_reg.write(bytes[0]);
			self.lba1_reg.write(bytes[1]);
			self.lba2_reg.write(bytes[2]);
			self.drive_reg.write(bytes[3]);
		}

		Ok(())
	}
}


lazy_static!
{
	pub static ref BUSES: Mutex<Vec<Bus>> = Mutex::new(Vec::new());
}


// Initialization
pub fn init()
{
	{
		let mut buses = BUSES.lock();
		buses.push(Bus::new(0, 0x1F0, 0x3F6, 14));
		buses.push(Bus::new(1, 0x170, 0x376, 15));
	}

	for drive in ls()
	{
		serprintln!("[INFO] ATA {}:{} {}\n", drive.bus, drive.disk, drive);
	}
}


// Drive struct
#[derive(Clone)]
pub struct Drive
{
	blk: u32,
	pub bus: u8,
	pub disk: u8,
	model: String,
	ser: String,
}


// Implementation of the drive struct
impl Drive
{
	// Block count
	pub fn blkcount(&self) -> u32
	{
		self.blk
	}


	// Block size
	pub const fn blksize(&self) -> u32
	{
		BLKSIZE as u32
	}


	// Open
	pub fn open(bus: u8, disk: u8) -> Option<Self>
	{
		let mut buses = BUSES.lock();
		if let Ok(IDResponse::ATA(res)) = buses[bus as usize].id_drive(disk)
		{
			let buf = res.map(u16::to_le_bytes).concat();
			let blk = u32::from_be_bytes(buf[120..124].try_into().unwrap()).rotate_left(16);
			let model = String::from_utf8_lossy(&buf[54..94]).trim().into();
			let ser = String::from_utf8_lossy(&buf[20..40]).trim().into();

			Some(Self { bus, disk, model, ser, blk })
		}
		else
		{
			None
		}
	}


	// Formatted size
	fn formatted_size(&self) -> (usize, String)
	{
		let count = self.blkcount() as usize;
		let size = self.blksize() as usize;
		let bytes = size * count;

		if bytes >> 20 < 1000
		{
			(bytes >> 20, String::from("MB"))
		}
		else
		{
			(bytes >> 30, String::from("GB"))
		}
	}
}


// List
pub fn ls() -> Vec<Drive>
{
	let mut res = Vec::new();
	for bus in 0..2
	{
		for disk in 0..2
		{
			if let Some(drive) = Drive::open(bus, disk)
			{
				res.push(drive)
			}
		}
	}
	res
}


// Read
pub fn read(bus: u8, drive: u8, blk: u32, buffer: &mut [u8]) -> Result<(), ()>
{
	let mut buses = BUSES.lock();
	buses[bus as usize].read(drive, blk, buffer)
}


// Write
pub fn write(bus: u8, drive: u8, blk: u32, buffer: &[u8]) -> Result<(), ()>
{
	let mut buses = BUSES.lock();
	buses[bus as usize].write(drive, blk, buffer)
}


// Implementation of fmt::Display for the Drive struct
impl fmt::Display for Drive
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		let (size, unit) = self.formatted_size();
		write!(f, "{} {} ({} {})", self.model, self.ser, size, unit)
	}
}
