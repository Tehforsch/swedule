use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
};

use crate::{grid::DependencyGraph, processor::Processor};

pub struct Processors {
    processors: Vec<Processor>,
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
        Processors { processors }
    }

    pub fn len(&self) -> usize {
        self.processors.len()
    }

    pub fn iter(&self) -> Box<dyn Iterator<Item = &Processor> + '_> {
        Box::new(self.processors.iter())
    }

    pub fn get_next_free(&mut self) -> &mut Processor {
        self.processors
            .iter_mut()
            .filter(|processor| !processor.asleep)
            .min_by_key(|processor| processor.time)
            .unwrap()
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
