use std::collections::VecDeque;

use generational_arena::Index;
use ordered_float::OrderedFloat;
use priority_queue::PriorityQueue;

use crate::param_file::ParamFile;
use crate::task::Task;
use crate::task_priority::TaskPriority;
use crate::vector_3d::Vector3D;

type TaskQueue = PriorityQueue<Index, TaskPriority>;
type SendQueue = VecDeque<(usize, Index)>;
type ReceiveQueue = PriorityQueue<Index, TaskPriority>;

#[derive(Debug)]
pub struct Processor {
    param_file: ParamFile,
    pub queue: TaskQueue,
    send_queue: SendQueue,
    receive_queue: ReceiveQueue,
    domain_center: Vector3D,
    pub num_solved: usize,
    pub time: OrderedFloat<f64>,
    pub num: usize,
    pub asleep: bool,
    pub time_spent_communicating: f64,
    pub time_spent_waiting: f64,
}

impl Processor {
    pub fn new(
        num: usize,
        queue: TaskQueue,
        domain_center: Vector3D,
        param_file: &ParamFile,
    ) -> Self {
        Processor {
            num,
            queue,
            domain_center,
            send_queue: SendQueue::new(),
            receive_queue: ReceiveQueue::new(),
            num_solved: 0,
            time: OrderedFloat(0.0),
            asleep: false,
            time_spent_waiting: 0.0,
            time_spent_communicating: 0.0,
            param_file: param_file.clone(),
        }
    }

    pub fn get_next_task(&mut self) -> Option<Index> {
        self.queue.pop().map(|(index, _)| index)
    }

    pub fn solve(&mut self, _task: &Task) {
        self.num_solved += 1;
        self.time += self.param_file.solve_time_offset;
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
        self.param_file.send_time_offset
            + num_sent as f64
                * self.param_file.send_time_per_byte
                * self.param_file.size_per_message
    }

    fn get_receive_time(&self, num_received: usize) -> f64 {
        self.get_send_time(num_received)
    }

    pub fn add_task_to_queue(&mut self, task_index: Index, priority: TaskPriority) {
        self.queue.push(task_index, priority);
    }

    pub fn add_task_to_send_queue(&mut self, task_index: Index, processor_num: usize) {
        self.send_queue.push_back((processor_num, task_index));
    }

    pub fn add_task_to_receive_queue(&mut self, task: Index, priority: TaskPriority) {
        self.receive_queue.push(task, priority);
    }

    pub fn go_to_sleep(&mut self) {
        self.asleep = true;
    }

    pub fn wake_up_at(&mut self, time: OrderedFloat<f64>) {
        if self.asleep {
            if *time > *self.time {
                self.time_spent_waiting += *time - *self.time;
                self.time = time;
            }
            self.asleep = false;
        }
    }
}
