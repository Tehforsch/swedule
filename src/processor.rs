use std::collections::VecDeque;

use generational_arena::Index;
use ordered_float::OrderedFloat;

use crate::{config, task::Task};

type TaskQueue = VecDeque<Index>;
type SendQueue = VecDeque<(usize, Index)>;
type ReceiveQueue = VecDeque<Index>;

#[derive(Debug)]
pub struct Processor {
    queue: TaskQueue,
    send_queue: SendQueue,
    receive_queue: ReceiveQueue,
    pub num_solved: usize,
    pub time: OrderedFloat<f64>,
    pub num: usize,
    pub asleep: bool,
}

impl Processor {
    pub fn new(num: usize, queue: TaskQueue) -> Self {
        Processor {
            num,
            queue,
            send_queue: SendQueue::new(),
            receive_queue: ReceiveQueue::new(),
            num_solved: 0,
            time: OrderedFloat(0.0),
            asleep: false,
        }
    }

    pub fn get_next_task(&mut self) -> Option<Index> {
        self.queue.pop_front()
    }

    pub fn solve(&mut self, _task: &Task) {
        self.num_solved += 1;
        self.time += config::SOLVE_TIME;
        // println!("{} solv {}", self.num, _task.cell.index);
    }

    pub fn send_tasks(&mut self) -> SendQueue {
        self.time += config::SEND_TIME;
        // for task in self.send_queue.iter() {
            // println!("{} send {:?}", self.num, task.1);
        // }
        self.send_queue.drain(..).collect()
    }

    pub fn receive_tasks(&mut self) -> usize {
        self.time += config::RECEIVE_TIME;
        let num_received = self.receive_queue.len();
        // for task in self.receive_queue.iter() {
            // println!("{} recv {:?}", self.num, task);
        // }
        self.queue.append(&mut self.receive_queue);
        num_received
    }

    pub fn add_task_to_queue(&mut self, task_index: Index) {
        self.queue.push_back(task_index);
    }

    pub fn add_task_to_send_queue(&mut self, task_index: Index, processor_num: usize) {
        self.send_queue.push_back((processor_num, task_index));
    }

    pub fn add_task_to_receive_queue(&mut self, task: Index) {
        self.receive_queue.push_back(task);
    }

    pub fn go_to_sleep(&mut self) {
        self.asleep = true;
    }

    pub fn wake_up(&mut self, time: OrderedFloat<f64>) {
        if self.asleep {
            self.time = time;
            self.asleep = false;
        }
    }
}
