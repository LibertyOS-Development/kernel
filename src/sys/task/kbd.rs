// src/task/kbd.rs
//
// Asynchronous keyboard implementation.

/*
	IMPORTS
*/

use conquer_once::spin::OnceCell;
use core::{pin::Pin, task::{Context, Poll}};
use crossbeam_queue::ArrayQueue;
use futures_util::{stream::{Stream, StreamExt}, task::AtomicWaker};
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, layouts, ScancodeSet1};

use crate::{print, println};



static SCANCODEQ: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();


impl Stream for ScancodeStream
{
	type Item = u8;

	fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>>
	{
		let queue = SCANCODEQ.try_get()
			.expect("[ERR] UNINITIALIZED");

		if let Ok(scancode) = queue.pop()
		{
			return Poll::Ready(Some(scancode));
		}

		WAKER.register(&cx.waker());
		match queue.pop()
		{
			Ok(scancode) =>
			{
				WAKER.take();
				Poll::Ready(Some(scancode))
			}
			Err(crossbeam_queue::PopError) => Poll::Pending,
		}
	}
}


pub struct ScancodeStream
{
	_private: (),
}

impl ScancodeStream
{
	pub fn new() -> Self
	{
		SCANCODEQ.try_init_once(|| ArrayQueue::new(100))
			.expect("[ERR] SCANCODESTREAM::NEW CAN ONLY BE CALLED ONCE");

		ScancodeStream
		{
			_private: ()
		}
	}
}

pub(crate) fn add_scancode(scancode: u8)
{
	if let Ok(queue) = SCANCODEQ.try_get()
	{
		if let Err(_) = queue.push(scancode)
		{
			println!("[WARN] SCANCODE QUEUE FULL; KEYBOARD INPUT DROPPED");
		}
		else
		{
			WAKER.wake();
		}
	}
	else
	{
		println!("[WARN] SCANCODE QUEUE UNINITIALIZED");
	}
}


pub async fn print_keypresses()
{
	let mut scancodes = ScancodeStream::new();
	let mut kbd = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);
	while let Some(scancode) = scancodes.next().await
	{
		if let Ok(Some(key_event)) = kbd.add_byte(scancode)
		{
			if let Some(key) = kbd.process_keyevent(key_event)
			{
				match key
				{
					DecodedKey::Unicode(character) => print!("{}", character),
					DecodedKey::RawKey(key) => print!("{:?}", key),
				}
			}
		}
	}
}
