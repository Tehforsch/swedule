use std::collections::VecDeque;

use generational_arena::Index;

use crate::{
    direction::Direction,
    grid::{DependencyGraph, Grid},
    processor::Processor,
    run_data::RunData,
};

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
            .map(|num| Processor::new(num, get_initial_queue(&graph, num)))
            .collect();
        Sweep { graph, processors }
    }

    pub fn run(&mut self) -> RunData {
        let num_to_solve = self.graph.len();
        loop {
            let processor = get_next_free_processor(&mut self.processors);
            let current_time = processor.time;
            let task_index = processor.get_next_task();
            if let Some(task_index) = task_index {
                handle_task_solving(&mut self.graph, processor, task_index);
            } else {
                let num_received = processor.receive_tasks();
                if num_received == 0 {
                    processor.go_to_sleep();
                }
                let sent_tasks = processor.send_tasks();
                for (processor_index, task) in sent_tasks {
                    self.processors[processor_index].add_task_to_receive_queue(task);
                    self.processors[processor_index].wake_up(current_time);
                }
            }
            if self.get_num_solved() == num_to_solve {
                break;
            }
        }
        RunData {
            time: *self
                .processors
                .iter()
                .map(|processor| processor.time)
                .max()
                .unwrap(),
            num_processors: self.processors.len(),
        }
    }

    pub fn get_num_solved(&self) -> usize {
        self.processors
            .iter()
            .map(|processor| processor.num_solved)
            .sum()
    }
}

fn handle_task_solving<'a>(
    graph: &mut DependencyGraph<'a>,
    processor: &mut Processor,
    task_index: Index,
) -> () {
    let task_node = graph.get(task_index).unwrap();
    let edge_indices: Vec<Index> = task_node.edges.iter().map(|edge| edge.index).collect();
    let task = &task_node.data;
    processor.solve(&task);
    for dependency_index in edge_indices.iter() {
        let downwind_task_node = graph.get_mut(*dependency_index).unwrap();
        let downwind_task = &mut downwind_task_node.data;
        downwind_task.num_upwind -= 1;
        if downwind_task.num_upwind == 0 {
            if downwind_task.processor_num == processor.num {
                processor.add_task_to_queue(downwind_task_node.index);
            } else {
                processor
                    .add_task_to_send_queue(downwind_task_node.index, downwind_task.processor_num);
            }
        }
    }
}

fn get_next_free_processor(processors: &mut [Processor]) -> &mut Processor {
    processors
        .iter_mut()
        .filter(|processor| !processor.asleep)
        .min_by_key(|processor| processor.time)
        .unwrap()
}

fn get_initial_queue<'a>(graph: &DependencyGraph<'a>, processor_num: usize) -> VecDeque<Index> {
    let mut queue = VecDeque::new();
    for task_node in graph.iter_nodes() {
        let task = &task_node.data;
        if task.processor_num == processor_num && task.num_upwind == 0 {
            queue.push_back(task_node.index);
        }
    }
    queue
}
