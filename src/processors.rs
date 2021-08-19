use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
};

use ordered_float::OrderedFloat;
use priority_queue::PriorityQueue;

use crate::{grid::DependencyGraph, processor::Processor, processor_priority::ProcessorPriority};
pub struct Processors {
    processors: Vec<Processor>,
    queue: PriorityQueue<usize, ProcessorPriority>,
}

impl Processors {
    pub fn new(graph: &DependencyGraph, num_processors: usize) -> Self {
        let mut processors: Vec<Processor> = (0..num_processors)
            .map(|num| Processor::new(num, VecDeque::new()))
            .collect();
        for task_node in graph.iter_nodes() {
            let task = &task_node.data;
            if task.num_upwind == 0 {
                processors[task.processor_num].add_task_to_queue(task_node.index);
            }
        }
        let queue = processors
            .iter()
            .map(|processor| Processors::get_queue_element(processor))
            .collect();
        Processors { processors, queue }
    }

    pub fn len(&self) -> usize {
        self.processors.len()
    }

    pub fn iter(&self) -> Box<dyn Iterator<Item = &Processor> + '_> {
        Box::new(self.processors.iter())
    }

    pub fn get_next_free(&mut self) -> &mut Processor {
        let (index, _) = self.queue.pop().unwrap();
        &mut self.processors[index]
    }

    pub fn reinsert_with_new_priority(&mut self, processor_num: usize) {
        let processor = &self.processors[processor_num];
        let (priority, item) = Processors::get_queue_element(processor);
        self.queue.push(priority, item);
    }

    fn get_queue_element(processor: &Processor) -> (usize, ProcessorPriority) {
        (
            processor.num,
            ProcessorPriority {
                time: -processor.time,
            },
        )
    }

    pub fn wake_up_at(&mut self, processor_num: usize, time: OrderedFloat<f64>) {
        self.processors[processor_num].wake_up_at(time);
        self.reinsert_with_new_priority(processor_num);
    }
}

impl Index<usize> for Processors {
    type Output = Processor;
    fn index(&self, index: usize) -> &Self::Output {
        &self.processors[index]
    }
}

impl IndexMut<usize> for Processors {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.processors[index]
    }
}
