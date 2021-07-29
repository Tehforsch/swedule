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
        for x in self.graph.iter_edges() {
            dbg!(x);
        }
        loop {
            let processor = get_next_free_processor(&mut self.processors);
            let task_index = processor.get_next_task();
            if let Some(task_index) = task_index {
                let task_node = self.graph.get(task_index).unwrap();
                let edge_indices: Vec<Index> = task_node.edges.iter().map(|edge| edge.index).collect();
                let task = &task_node.label;
                processor.solve(&task);
                for dependency_index in edge_indices.iter() {
                    let downwind_task_node = self.graph.get_mut(*dependency_index).unwrap();
                    let downwind_task = &mut downwind_task_node.label;
                    downwind_task.num_upwind -= 1;
                    if downwind_task.num_upwind == 0 {
                        processor.add_task_to_queue(downwind_task_node.index);
                    }
                }
            }
            else {
                processor.send_tasks();
                processor.receive_tasks();
            }
            let num_solved: usize = self.processors.iter().map(|processor| processor.num_solved).sum();
            if num_solved == self.graph.len() {
                break;
            }
        }
    }
}

fn get_next_free_processor(processors: &mut [Processor]) -> &mut Processor {
    processors.iter_mut().min_by_key(|processor| processor.time).unwrap()
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
