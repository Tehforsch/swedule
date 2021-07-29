use std::collections::VecDeque;

use generational_arena::Index;
use ordered_float::OrderedFloat;

use crate::{config, task::Task};

type TaskQueue = VecDeque<Index>;

pub struct Processor {
    queue: TaskQueue,
    pub num_solved: usize,
    pub time: OrderedFloat<f64>,
}

impl Processor {
    pub fn new(queue: TaskQueue) -> Self {
        Processor {
            queue,
            num_solved: 0,
            time: OrderedFloat(0.0),
        }
    }

    pub fn get_next_task(&mut self) -> Option<Index> {
        self.queue.pop_front()
    }

    pub fn solve(&mut self, _task: &Task) {
        self.num_solved += 1;
        self.time += config::SOLVE_TIME;
    }

    pub fn send_tasks(&mut self) {
        self.time += config::SEND_TIME;
    }

    pub fn receive_tasks(&mut self) {
        self.time += config::RECEIVE_TIME;
    }

    pub fn add_task_to_queue(&mut self, task_index: Index) {
        self.queue.push_back(task_index);
    }
}
