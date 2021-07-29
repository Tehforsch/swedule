use std::collections::VecDeque;

use generational_arena::Index;

use crate::{
    direction::Direction,
    grid::{DependencyGraph, Grid},
    processor::Processor,
};

struct RunData {
    time: f64,
}

pub struct Sweep<'a> {
    graph: DependencyGraph<'a>,
    processors: Vec<Processor>,
}

impl<'a> Sweep<'a> {
    pub fn new(grid: &'a Grid, directions: &[Direction], num_processors: usize) -> Self {
        let graph: DependencyGraph = directions
            .iter()
            .map(|dir| grid.get_dependency_graph(dir))
            .collect();
        let processors = (0..num_processors)
            .map(|num| Processor::new(get_initial_queue(&graph, num)))
            .collect();
        Sweep { graph, processors }
    }

    pub fn run(&mut self) {
        loop {
            for processor in self.processors.iter_mut() {
                let task_index = processor.get_next_task();
                if let Some(task_index) = task_index {
                    let task_node = self.graph.get(task_index).unwrap();
                    processor.solve(&task_node.label);
                }
            }
        }
    }
}

fn get_initial_queue<'a>(graph: &DependencyGraph<'a>, processor_num: usize) -> VecDeque<Index> {
    let mut queue = VecDeque::new();
    for task_node in graph.iter_nodes() {
        let task = &task_node.label;
        if task.processor_num == processor_num && task.num_upwind == 0 {
            queue.push_back(task_node.index);
        }
    }
    queue
}
