use std::collections::VecDeque;

use generational_arena::Index;
use ordered_float::OrderedFloat;

use crate::task::Task;
use crate::config::*;

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
    pub time_spent_communicating: f64,
    pub time_spent_waiting: f64,
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
            time_spent_waiting: 0.0,
            time_spent_communicating: 0.0,
        }
    }

    pub fn get_next_task(&mut self) -> Option<Index> {
        self.queue.pop_front()
    }

    pub fn solve(&mut self, _task: &Task) {
        self.num_solved += 1;
        self.time += SOLVE_TIME;
    }

    pub fn send_tasks(&mut self) -> SendQueue {
        let sent_tasks: SendQueue = self.send_queue.drain(..).collect();
        let send_time = self.get_send_time(sent_tasks.len());
        self.time_spent_communicating += send_time;
        self.time += send_time;
        sent_tasks
    }

    pub fn receive_tasks(&mut self) -> usize {
        let num_received = self.receive_queue.len();
        let receive_time = self.get_receive_time(num_received);
        self.time_spent_communicating += receive_time;
        self.time += receive_time;
        self.queue.append(&mut self.receive_queue);
        num_received
    }

    fn get_send_time(&self, num_sent: usize) -> f64 {
        SEND_TIME_OFFSET_SECONDS + num_sent as f64 * SEND_TIME_PER_BYTE_SECONDS * SIZE_PER_MESSAGE
    }

    fn get_receive_time(&self, num_received: usize) -> f64 {
        self.get_send_time(num_received)
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
            self.time_spent_waiting += *time - *self.time;
            self.time = time;
            self.asleep = false;
        }
    }
}
