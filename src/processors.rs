use std::ops::Index;
use std::ops::IndexMut;

use ordered_float::OrderedFloat;
use priority_queue::PriorityQueue;

use crate::grid::DependencyGraph;
use crate::param_file::ParamFile;
use crate::processor::Processor;
use crate::processor_priority::ProcessorPriority;
use crate::vector_3d::Vector3D;
pub struct Processors {
    processors: Vec<Processor>,
    queue: PriorityQueue<usize, ProcessorPriority>,
}

impl Processors {
    pub fn new(graph: &DependencyGraph, num_processors: usize, param_file: &ParamFile) -> Self {
        let centers = Processors::get_centers(graph, num_processors);
        let mut processors: Vec<Processor> = centers
            .into_iter()
            .enumerate()
            .map(|(num, center)| Processor::new(num, PriorityQueue::new(), center, param_file))
            .collect();
        for task_node in graph.iter_nodes() {
            let task = &task_node.data;
            let priority = task.get_priority();
            if task.num_upwind == 0 {
                processors[task.processor_num].add_task_to_queue(task_node.index, priority);
            }
        }
        let queue = processors
            .iter()
            .map(|processor| Processors::get_queue_element(processor))
            .collect();
        Processors { processors, queue }
    }

    pub fn get_centers(graph: &DependencyGraph, num_processors: usize) -> Vec<Vector3D> {
        let mut centers: Vec<Vector3D> = (0..num_processors)
            .map(|_| Vector3D::new(0., 0., 0.))
            .collect();
        let mut num_cells: Vec<usize> = (0..num_processors).map(|_| 0).collect();
        for task_node in graph.iter_nodes() {
            centers[task_node.data.processor_num] += &task_node.data.cell.center;
            num_cells[task_node.data.processor_num] += 1;
        }
        for (mut center, num) in centers.iter_mut().zip(num_cells) {
            center /= num as f64;
        }
        centers
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
