// src/libcore/dev/kbd/mod.rs
//
// This is the mod.rs file for the libcore::dev::kbd module.


/*
	IMPORTS
*/

use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, Error, HandleControl, KeyCode, KeyEvent, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::Port;



// Lazy static wrapper around the the static reference KBD
lazy_static!
{
	pub static ref KBD: Mutex<Option<KbdLayout>> = Mutex::new(None);
}


// Keyboard layout enumeration
pub enum KbdLayout
{
	// AZERTY layout
	AZERTY(Keyboard<layouts::Azerty, ScancodeSet1>),

	// DVORAK layout
	DVORAK(Keyboard<layouts::Dvorak104Key, ScancodeSet1>),

	// QWERTY layout
	QWERTY(Keyboard<layouts::Us104Key, ScancodeSet1>),
}


// Implementation of the KbdLayout enumeration
impl KbdLayout
{
	// Add byte
	fn addbyte(&mut self, scancode: u8) -> Result<Option<KeyEvent>, Error>
	{
		match self
		{
			// AZERTY
			KbdLayout::AZERTY(keyboard) => keyboard.add_byte(scancode),

			// DVORAK
			KbdLayout::DVORAK(keyboard) => keyboard.add_byte(scancode),

			// QWERTY
			KbdLayout::QWERTY(keyboard) => keyboard.add_byte(scancode),
		}
	}


	// From
	fn from(name: &str) -> Option<Self>
	{
		// Match layout name to layout
		match name
		{
			// AZERTY
			"azerty" => Some(KbdLayout::AZERTY(Keyboard::new(layouts::Azerty, ScancodeSet1, HandleControl::MapLettersToUnicode))),

			// DVORAK
			"dvorak" => Some(KbdLayout::DVORAK(Keyboard::new(layouts::Dvorak104Key, ScancodeSet1, HandleControl::MapLettersToUnicode))),

			// QWERTY
			"qwerty" => Some(KbdLayout::QWERTY(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::MapLettersToUnicode))),

			_ => None,
		}
	}


	// Process keyevent
	fn proc_keyevent(&mut self, keyevent: KeyEvent) -> Option<DecodedKey>
	{
		match self
		{
			// AZERTY
			KbdLayout::AZERTY(keyboard) => keyboard.process_keyevent(keyevent),

			// DVORAK
			KbdLayout::DVORAK(keyboard) => keyboard.process_keyevent(keyevent),

			// QWERTY
			KbdLayout::QWERTY(keyboard) => keyboard.process_keyevent(keyevent),
		}
	}
}


// Initialization
pub fn init()
{
	// TODO: Make the keyboard layout reflect user-configuration
	setkbd("qwerty");
	crate::libcore::sys::idt::set_irh(1, intrh);
}


// Interrupt handler
fn intrh()
{
	if let Some(ref mut keyboard) = *KBD.lock()
	{
		let scancode = read_scancode();

		if let Ok(Some(key_event)) = keyboard.addbyte(scancode)
		{
			if let Some(key) = keyboard.proc_keyevent(key_event)
			{
				match key
				{
					// Unicode character
					DecodedKey::Unicode(c) => sendkey(c),

					// Up arrow
					DecodedKey::RawKey(KeyCode::ArrowUp) => sendcsi('A'),

					// Down arrow
					DecodedKey::RawKey(KeyCode::ArrowDown) => sendcsi('B'),

					// Right arrow
					DecodedKey::RawKey(KeyCode::ArrowRight) => sendcsi('C'),

					// Left arrow
					DecodedKey::RawKey(KeyCode::ArrowLeft) => sendcsi('D'),

					_ => {},
				};
			}
		}
	}
}

// Read scancode
fn read_scancode() -> u8
{
	// Create a port, at 0x60
	let mut port = Port::new(0x60);

	// Read from the aforementioned port
	unsafe
	{
		port.read()
	}
}


// Send CSI
fn sendcsi(c: char)
{
	sendkey('\x1B');
	sendkey('[');
	sendkey(c);
}


// Send key
fn sendkey(c: char)
{
	crate::libcore::sys::console::keyhandler(c);
}


// Set keyboard
pub fn setkbd(layout: &str) -> bool
{
	if let Some(keyboard) = KbdLayout::from(layout)
	{
		*KBD.lock() = Some(keyboard);
		true
	}
	else
	{
		false
	}
}
