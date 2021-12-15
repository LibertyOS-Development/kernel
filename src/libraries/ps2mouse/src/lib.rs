// This library is used to give the kernel access to a PS2 mouse.

#![no_std]
#![feature(const_fn_fn_ptr_basics)]

use bitflags::bitflags;
use x86_64::instructions::port::Port;

const ADDRPORT_ADDR: u16 = 0x64;
const DATAPORT_ADDR: u16 = 0x60;
const GETSTAT_BYTE: u8 = 0x20;
const SETSTAT_BYTE: u8 = 0x60;

bitflags!
{
	#[derive(Default)]
	pub struct MFlags: u8
	{
		const LEFT_BTN = 0b0000_0001;
		const RIGHT_BTN = 0b0000_0010;
		const MIDDLE_BTN = 0b0000_0100;
		const ALWAYS_ONE = 0b0000_1000;
		const XSIGN = 0b0001_0000;
		const YSIGN = 0b0010_0000;
		const XOF = 0b0100_0000;
		const YOF = 0b1000_0000;
	}
}

#[repr(u8)]
enum CMD
{
	EnablePacketStream = 0xF4,
	SetDef = 0xF6,
}

#[derive(Debug)]
pub struct Mouse
{
	cmdport: x86_64::instructions::port::Port<u8>,
	dataport: x86_64::instructions::port::Port<u8>,
	currpacket: u8,
	currstat: MStat,
	complstat: MStat,
	complete: Option<fn(MStat)>,
}

impl Default for Mouse
{
	fn default() -> Mouse
	{
		Mouse::new()
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct MStat
{
	flags: MFlags<u8>,
	x: i16,
	y: i16,
}

impl MStat
{
	pub const fn new() -> MStat
	{
		MStat
		{
			flags: MFlags::empty(),
			x: 0,
			y: 0,
		}
	}

	pub fn leftbtn_down(&self) -> bool
	{
		self.flags.contains(MFlags::LEFT_BTN)
	}

	pub fn leftbtn_up(&self) -> bool
	{
		!self.flags.contains(MFlags::LEFT_BTN)
	}

	pub fn rightbtn_down(&self) -> bool
	{
		self.flags.contains(MFlags::RIGHT_BTN)
	}

	pub fn rightbtn_up(&self) -> bool
	{
		!self.flags.contains(MFlags::RIGHT_BTN)
	}

	pub fn xmoved(&self) -> bool
	{
		self.x != 0
	}

	pub fn ymoved(&self) -> bool
	{
		self.y != 0
	}

	pub fn moved(&self) -> bool
	{
		self.xmoved() || self.ymoved()
	}

	pub fn getx(&self) -> i16
	{
		self.x
	}

	pub fn gety(&self) -> i16
	{
		self.y
	}
}


impl Mouse
{
	pub const fn new() -> Mouse
	{
		Mouse
		{
			cmdport: x86_64::instructions::port::Port::new(ADDRPORT_ADDR),
			dataport: x86_64::instructions::port::ort::new(DATAPORT_ADDR),
			currpacket: 0,
			currstat: MStat::new(),
			complstat: MStat::new(),
			complete: None,
		}
	}

	pub fn getstat(&self) -> MStat
	{
		self.complstat
	}

	pub fn init(&mut self) -> Result<(), &'static str>
	{
		self.write_cmdport(GETSTAT_BYTE)?;
		let stat = self.read_dataport()? | 0x02;
		self.write_cmdport(SETSTAT_BYTE)?;
		self.write_dataport(stat & 0xDF)?;
		self.sendcmd(CMD::SetDef)?;
		self.sendcmd(CMD::EnablePacketStream)?;
		Ok(())
	}

	pub fn procpacket(&mut self, packet: u8)
	{
		match self.currpacket
		{
			0 =>
			{
				let flags = MFlags::from_bits_truncate(packet);
				if !flags.contains(MFlags::ALWAYS_ONE)
				{
					return;
				}
				self.currstat.flags = flags;
			}
			1 => self.proc_xmv(packet),
			2 => {
				self.proc_ymv(packet);
				self.complstat = self.currstat;
				if let Some(complete) = self.complete
				{
					complete(self.complstat);
				}
			}
			_ => unreachable!(),
		}
		self.currpacket = (self.currpacket + 1) % 3;
	}

	pub fn set_on_complete(&mut self, handler: fn(MStat))
	{
		self.complete = Some(handler);
	}

	fn proc_xmv(&mut self, packet: u8)
	{
		if !self.currstat.flags.contains(MFlags::XOF)
		{
			self.currstat.x = if self.currstat.flags.contains(MFlags::XSIGN)
			{
				self.signext(packet)
			}
			else
			{
				packet as i16
			};
		}
	}

	fn proc_ymv(&mut self, packet: u8)
	{
		if !self.currstat.flags.contains(MFlags::YOF)
		{
			self.currstat.y = if self.currstat.flags.contains(MFlags::YSIGN)
			{
				self.signext(packet)
			}
			else
			{
				packet as i16
			};
		}
	}

	fn read_dataport(&mut self) -> Result<u8, &'static str>
	{
		self.wait_for_read()?;
		Ok(unsafe { self.dataport.read() })
	}

	fn sendcmd(&mut self, cmd: CMD) -> Result<(), &'static str>
	{
		self.write_cmdport(0xD4)?;
		self.write_dataport(cmd as u8)?;
		if self.read_dataport()? != 0xFA
		{
			return Err("[ERR] MOUSE UNRESPONSIVE TO COMMAND");
		}
		Ok(())
	}

	fn signext(&self, packet: u8) -> i16
	{
		((packet as u16) | 0xFF00) as i16
	}

	fn write_cmdport(&mut self, val: u8) -> Result<(), &'static str>
	{
		self.wait_for_write()?;
		unsafe
		{
			self.cmdport.write(val);
		}
		Ok(())
	}

	fn write_dataport(&mut self, val: u8) -> Result<(), &'static str>
	{
		self.wait_for_write()?;
		unsafe
		{
			self.dataport.write(val);
		}
		Ok(())
	}

	fn wait_for_read(&mut self) -> Result<(), &'static str>
	{
		let timeout = 100_000;
		for _ in 0..timeout
		{
			let val = unsafe { self.cmdport.read() };
			if (val & 0x1) == 0x1
			{
				return Ok(());
			}
		}
		Err("[ERR] MOUSE READ TIMEOUT")
	}

	fn wait_for_write(&mut self) -> Result<(), &'static str>
	{
		let timeout = 100_000;
		for _ in 0..timeout
		{
			let val = unsafe { self.cmdport.read() };
			if (val & 0x2) == 0x0
			{
				return Ok(());
			}
		}
		Err("[ERR] MOUSE WRITE TIMEOUT")
	}
}
