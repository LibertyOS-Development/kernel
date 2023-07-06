// src/dev/mouse.rs
//
// This should allow the kernel to work with PS2-mice, but there is no GUI yet. :(
// Thank you to github.com/rustos-dev.

use bitflags::bitflags;
use x86_64::instructions::port::Port;


const ADDR_PORT_ADDR: u16 = 0x64;
const DATA_PORT_ADDR: u16 = 0x60;
const GET_STAT_BYTE: u8 = 0x20;
const SET_STAT_BYTE: u8 = 0x60;


bitflags!
{
	#[derive(Default)]
	pub struct MouseFlags: u8
	{
		const LEFT_BTN = 0b0000_0001;
		const RIGHT_BTN = 0b0000_0010;
		const MIDDLE_BTN = 0b0000_0100;
		const ALWAYS_ONE = 0b0000_1000;
		const X_SIGN = 0b0001_0000;
		const Y_SIGN = 0b0010_0000;
		const X_OVERFLOW = 0b0100_0000;
		const Y_OVERFLOW = 0b1000_0000;
	}
}


#[repr(u8)]
enum Cmd
{
	EnablePacketStreaming = 0xF4,
	SetDef = 0xF6,
}


#[derive(Debug)]
pub struct Mouse
{
	cmdport: Port<u8>,
	dataport: Port<u8>,
	currentpacket: u8,
	currentstate: MouseState,
	completedstate: MouseState,
	on_complete: Option<fn(MouseState)>,
}

impl Default for Mouse
{
	fn default() -> Mouse
	{
		Mouse::new()
	}
}


#[derive(Debug, Copy, Clone, Default)]
pub struct MouseState
{
	flags: MouseFlags,
	x: i16,
	y: i16,
}


impl MouseState
{
	pub const fn new() -> MouseState
	{
		MouseState
		{
			flags: MouseFlags::empty(),
			x: 0,
			y: 0,
		}
	}

	pub fn left_btn_down(&self) -> bool
	{
		self.flags.contains(MouseFlags::LEFT_BTN)
	}

	pub fn left_btn_up(&self) -> bool
	{
		!self.flags.contains(MouseFlags::LEFT_BTN)
	}

	pub fn right_btn_down(&self) -> bool
	{
		self.flags.contains(MouseFlags::RIGHT_BTN)
	}

	pub fn right_btn_up(&self) -> bool
	{
		!self.flags.contains(MouseFlags::RIGHT_BTN)
	}

	pub fn x_moved(&self) -> bool
	{
		self.x != 0
	}

	pub fn y_moved(&self) -> bool
	{
		self.y != 0
	}

	pub fn moved(&self) -> bool
	{
		self.x_moved() || self.y_moved()
	}

	pub fn get_x(&self) -> i16
	{
		self.x
	}

	pub fn get_y(&self) -> i16
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
			cmdport: Port::new(ADDR_PORT_ADDR),
			dataport: Port::new(DATA_PORT_ADDR),
			currentpacket: 0,
			currentstate: MouseState::new(),
			completedstate: MouseState::new(),
			on_complete: None,
		}
	}

	pub fn get_state(&self) -> MouseState
	{
		self.completedstate
	}

	pub fn init(&mut self) -> Result<(), &'static str>
	{
		self.write_cmd_port(GET_STAT_BYTE)?;
		let status = self.read_data_port()? | 0x02;
		self.write_cmd_port(SET_STAT_BYTE)?;
		self.write_data_port(status & 0xDF)?;
		self.send_cmd(Cmd::SetDef)?;
		self.send_cmd(Cmd::EnablePacketStreaming)?;

		Ok(())
	}

	pub fn proc_packet(&mut self, packet: u8)
	{
		match self.currentpacket
		{
			0 =>
			{
				let flags = MouseFlags::from_bits_truncate(packet);
				if !flags.contains(MouseFlags::ALWAYS_ONE)
				{
					return;
				}

				self.currentstate.flags = flags;
			}

			1 => self.proc_x_movement(packet),

			2 =>
			{
				self.proc_y_movement(packet);
				self.completedstate = self.currentstate;
				if let Some(on_complete) = self.on_complete
				{
					on_complete(self.completedstate);
				}
			}

			_ => unreachable!(),
		}

		self.currentpacket = (self.currentpacket + 1) % 3;
	}

	pub fn set_on_complete(&mut self, handler: fn(MouseState))
	{
		self.on_complete = Some(handler);
	}

	fn proc_x_movement(&mut self, packet: u8)
	{
		if !self.currentstate.flags.contains(MouseFlags::X_OVERFLOW)
		{
			self.currentstate.x = if self.currentstate.flags.contains(MouseFlags::X_SIGN)
			{
				self.sign_ext(packet)
			}
			else
			{
				packet as i16
			};
		}
	}


	fn proc_y_movement(&mut self, packet: u8)
	{
		if !self.currentstate.flags.contains(MouseFlags::Y_OVERFLOW)
		{
			self.currentstate.y = if self.currentstate.flags.contains(MouseFlags::Y_SIGN)
			{
				self.sign_ext(packet)
			}
			else
			{
				packet as i16
			};
		}
	}

	fn read_data_port(&mut self) -> Result<u8, &'static str>
	{
		self.wait_for_read()?;
		Ok(unsafe { self.dataport.read() })
	}

	fn send_cmd(&mut self, cmd: Cmd) -> Result<(), &'static str>
	{
		self.write_cmd_port(0xD4)?;
		self.write_data_port(cmd as u8)?;

		if self.read_data_port()? != 0xFA
		{
			return Err("[ERR] MOUSE IS UNRESPONSIVE");
		}

		Ok(())
	}

	fn sign_ext(&self, packet: u8) -> i16
	{
		((packet as u16) | 0xFF00) as i16
	}

	fn write_cmd_port(&mut self, value: u8) -> Result<(), &'static str>
	{
		self.wait_for_write()?;

		unsafe
		{
			self.cmdport.write(value);
		}

		Ok(())
	}

	fn write_data_port(&mut self, value: u8) -> Result<(), &'static str>
	{
		self.wait_for_write()?;

		unsafe
		{
			self.dataport.write(value);
		}

		Ok(())
	}

	fn wait_for_read(&mut self) -> Result<(), &'static str>
	{
		let timeout = 100_000;
		for _ in 0..timeout
		{
			let value = unsafe
			{
				self.cmdport.read()
			};

			if (value & 0x1) == 0x1
			{
				return Ok(());
			}
		}

		Err("[ERR] WAIT FOR MOUSE READ TIMEOUT")
	}


	fn wait_for_write(&mut self) -> Result<(), &'static str>
	{
		let timeout = 100_000;
		for _ in 0..timeout
		{
			let value = unsafe
			{
				self.cmdport.read()
			};

			if (value & 0x2) == 0x0
			{
				return Ok(());
			}
		}

		Err("[ERR] WAIT FOR MOUSE WRITE TIMEOUT")
	}
}
