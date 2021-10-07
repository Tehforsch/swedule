use generational_arena::Index;

use crate::direction::Direction;
use crate::grid::DependencyGraph;
use crate::grid::Grid;
use crate::param_file::ParamFile;
use crate::processor::Processor;
use crate::processors::Processors;
use crate::run_data::RunData;

pub struct Sweep<'a> {
    graph: DependencyGraph<'a>,
    processors: Processors,
    param_file: ParamFile,
}

impl<'a> Sweep<'a> {
    pub fn new(
        param_file: &ParamFile,
        grid: &'a Grid,
        directions: &[Direction],
        num_processors: usize,
    ) -> Self {
        let graph: DependencyGraph = directions
            .iter()
            .map(|dir| grid.get_dependency_graph(dir))
            .collect();
        let processors = Processors::new(&graph, num_processors, param_file);

        Sweep {
            graph,
            processors,
            param_file: param_file.clone(),
        }
    }

    pub fn run(&mut self) -> RunData {
        let mut num_to_solve = self.graph.len();
        let mut num_solved_without_sending = 0;
        loop {
            let processor = &mut self.processors.get_next_free();
            let processor_num = processor.num;
            let current_time = processor.time;
            let task_index = processor.get_next_task();
            let mut asleep = false;
            if let Some(task_index) = task_index {
                handle_task_solving(&mut self.graph, processor, task_index);
                num_to_solve -= 1;
                num_solved_without_sending += 1;
            }
            if task_index.is_none() || num_solved_without_sending >= self.param_file.batch_size {
                num_solved_without_sending = 0;
                let num_received = processor.receive_tasks();
                if num_received == 0 && task_index.is_none() {
                    asleep = true;
                    processor.go_to_sleep();
                }
                let sent_tasks = processor.send_tasks();
                for (processor_index, task) in sent_tasks {
                    let priority = self.graph.get(task).unwrap().data.get_priority();
                    self.processors[processor_index].add_task_to_receive_queue(task, priority);
                    self.processors.wake_up_at(processor_index, current_time);
                }
            }
            if num_to_solve == 0 {
                break;
            }
            if !asleep {
                self.processors.reinsert_with_new_priority(processor_num);
            }
        }
        RunData::new(&self.processors)
    }
}

fn handle_task_solving<'a>(
    graph: &mut DependencyGraph<'a>,
    processor: &mut Processor,
    task_index: Index,
) {
    let task_node = graph.get(task_index).unwrap();
    let edge_indices: Vec<Index> = task_node.edges.iter().map(|edge| edge.index).collect();
    let task = &task_node.data;
    processor.solve(task);
    for dependency_index in edge_indices.iter() {
        let downwind_task_node = graph.get_mut(*dependency_index).unwrap();
        let downwind_task = &mut downwind_task_node.data;
        let priority = downwind_task.get_priority();
        downwind_task.num_upwind -= 1;
        if downwind_task.num_upwind == 0 {
            if downwind_task.processor_num == processor.num {
                processor.add_task_to_queue(downwind_task_node.index, priority);
            } else {
                processor
                    .add_task_to_send_queue(downwind_task_node.index, downwind_task.processor_num);
            }
        }
    }
}
