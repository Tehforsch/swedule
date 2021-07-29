use std::collections::VecDeque;

use generational_arena::Index;

use crate::task::Task;

type TaskQueue = VecDeque<Index>;

pub struct Processor {
    queue: TaskQueue,
    num_solved: usize,
}

impl Processor {
    pub fn new(queue: TaskQueue) -> Self {
        Processor {
            queue,
            num_solved: 0,
        }
    }

    pub fn get_next_task(&mut self) -> Option<Index> {
        self.queue.pop_front()
    }

    pub fn solve(&mut self, task_: &Task) {
        self.num_solved += 1;
    }
}
