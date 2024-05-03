use super::Task;
use core::task::{Waker, RawWaker};
use alloc::{collections::VecDeque, vec::Vec};

pub struct SimpleExecutor {
    task_queue: VecDeque<Task>
}

impl SimpleExecutor {
    pub fn new() -> SimpleExecutor {
        SimpleExecutor {
            task_queue: VecDeque::new(),
        }
    }

    pub fn spawn(&mut self, task: Task) {
        self.task_queue.push_back(task)
    }
}

fn dummy_raw_waker() -> RawWaker {
    todo!();
}

fn dummy_waker() -> Waler {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}
