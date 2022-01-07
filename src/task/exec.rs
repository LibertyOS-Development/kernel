// src/task/exec.rs
//
// Dedicated module for the Exec type.

/*
	IMPORTS
*/

use alloc::{collections::BTreeMap, sync::Arc, task::Wake};
use core::task::{Context, Poll, Waker};
use crossbeam_queue::ArrayQueue;

use super::{Task, TaskID};


// Exec struct
pub struct Exec
{
	tasks: BTreeMap<TaskID, Task>,
	taskq: Arc<ArrayQueue<TaskID>>,
	waker_cache: BTreeMap<TaskID, Waker>,
}


// Implementation of the Exec struct
impl Exec
{
	pub fn new() -> Self
	{
		Exec
		{
			tasks: BTreeMap::new(),
			taskq: Arc::new(ArrayQueue::new(100)),
			waker_cache: BTreeMap::new(),
		}
	}

	pub fn run(&mut self) -> !
	{
		loop
		{
			self.run_ready_tasks();
			self.sleep_if_idle();
		}
	}

	fn sleep_if_idle(&self)
	{
		use x86_64::instructions::interrupts::{self, enable_and_hlt};

		interrupts::disable();

		if self.taskq.is_empty()
		{
			enable_and_hlt();
		}
		else
		{
			interrupts::enable();
		}
	}

	pub fn spawn(&mut self, task: Task)
	{
		let taskid = task.id;
		if self.tasks.insert(task.id, task).is_some()
		{
			// Returned if there is a TaskID conflict
			panic!("[ERR] TASK-ID IS ALREADY BEING USED BY ANOTHER TASK");
		}
	self.taskq.push(taskid)
		.expect("[ERR] QUEUE FULL");
	}


	fn run_ready_tasks(&mut self)
	{
		let Self
		{
			tasks,
			taskq,
			waker_cache,
		} = self;

		while let Ok(taskid) = taskq.pop()
		{
			let task = match tasks.get_mut(&taskid)
			{
				Some(task) => task,
				// Task does not exist anymore
				None => continue,
			};

			let waker = waker_cache
				.entry(taskid)
				.or_insert_with(|| TaskWaker::new(taskid, taskq.clone()));

			let mut context = Context::from_waker(waker);

			match task.poll(&mut context)
			{
				Poll::Ready(()) =>
				{
					// When task has been completed, remove said task and its cached waker
					tasks.remove(&taskid);
					waker_cache.remove(&taskid);
				}

				Poll::Pending => {}
			}
		}
	}
}


// The TaskWaker struct
struct TaskWaker
{
	taskid: TaskID,
	taskq: Arc<ArrayQueue<TaskID>>,
}


// Implementation of the TaskWaker struct
impl TaskWaker
{
	fn awaken_task(&self)
	{
		self.taskq.push(self.taskid)
			.expect("[ERR] TASKQ FULL");
	}

	fn new(taskid: TaskID, taskq: Arc<ArrayQueue<TaskID>>) -> Waker
	{
		Waker::from(Arc::new(TaskWaker
		{
			taskid,
			taskq,
		}))
	}
}


// Implementation of the Wake trait for TaskWaker
impl alloc::task::Wake for TaskWaker
{
	fn wake(self: Arc<Self>)
	{
		self.awaken_task();
	}

	fn wake_by_ref(self: &Arc<Self>)
	{
		self.awaken_task();
	}
}
