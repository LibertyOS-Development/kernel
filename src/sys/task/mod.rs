// src/task/mod.rs
//
// This is the mod.rs file for task module.

/*
	IMPORTS
*/

use alloc::boxed::Box;
use core::{future::Future, pin::Pin, sync::atomic::{AtomicU64, Ordering}, task::{Context, Poll}};

pub mod exec;
pub mod kbd;
pub mod simpexec;


// Task struct
pub struct Task
{
	future: Pin<Box<dyn Future<Output = ()>>>,
	id: TaskID,
}


// Implementation of the task struct
impl Task
{
	pub fn new(future: impl Future<Output = ()> + 'static) -> Task
	{
		Task
		{
			future: Box::pin(future),
			id: TaskID::new(),
		}
	}

	// Enables executor to poll stored future
	fn poll(&mut self, context: &mut Context) -> Poll<()>
	{
		self.future.as_mut().poll(context)
	}
}


// TaskID
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskID(u64);


// Implementation of TaskID
impl TaskID
{
	fn new() -> Self
	{
		static NEXTID: AtomicU64 = AtomicU64::new(0);
		TaskID(NEXTID.fetch_add(1, Ordering::Relaxed))
	}
}
