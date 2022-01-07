// src/task/simpexec.rs
//
// This module provides the kernel with a simple implementation of an executor.


/*
	IMPORTS
*/

use alloc::collections::VecDeque;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use super::Task;


// The main structure for the simpexec module
pub struct SimpleExec
{
	taskq: VecDeque<Task>,
}


// Implementation of the SimpleExec structure
impl SimpleExec
{
	pub fn new() -> SimpleExec
	{
		SimpleExec
		{
			taskq: VecDeque::new(),
		}
	}


	pub fn run(&mut self)
	{
		while let Some(mut task) = self.taskq.pop_front()
		{
			let waker = dummy_waker();
			let mut context = Context::from_waker(&waker);
			match task.poll(&mut context)
			{
				// Task completed
				Poll::Ready(()) => {}
				Poll::Pending => self.taskq.push_back(task),
			}
		}
	}

	pub fn spawn(&mut self, task: Task)
	{
		self.taskq.push_back(task)
	}
}


fn dummy_raw_waker() -> RawWaker
{
	fn nop(_: *const ()) {}
	fn clone(_: *const ()) -> RawWaker
	{
		dummy_raw_waker()
	}

	let vtab = &RawWakerVTable::new(clone, nop, nop, nop);
	RawWaker::new(0 as *const (), vtab)
}

fn dummy_waker() -> Waker
{
	unsafe
	{
		Waker::from_raw(dummy_raw_waker())
	}
}
