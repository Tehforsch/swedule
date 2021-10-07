use generational_arena::Index;
use ordered_float::OrderedFloat;

use crate::cell::Cell;
use crate::dependency::Dependency;
use crate::direction::Direction;
use crate::face::Face;
use crate::graph::Graph;
use crate::node::Node;
use crate::task::Task;

pub type DependencyGraph<'a> = Graph<Task<'a>, Dependency>;

pub struct Grid {
    data: Graph<Cell, Face>,
}

impl Grid {
    pub fn get_dependency_graph<'b>(&self, direction: &'b Direction) -> DependencyGraph<'_> {
        let mut tasks: Vec<Task> = self
            .data
            .iter()
            .map(|cell| Task {
                cell,
                direction: direction.clone(),
                processor_num: cell.processor_num,
                num_upwind: 0,
            })
            .collect();
        let mut dependency_data = vec![];
        for (upwind_cell, downwind_cell, face) in self.data.iter_edges() {
            if Grid::is_downwind(face, direction) {
                dependency_data.push((
                    upwind_cell.global_index,
                    downwind_cell.global_index,
                    Dependency,
                ));
                tasks[downwind_cell.global_index].num_upwind += 1;
            }
        }
        Graph::from_nodes_and_edge_list(tasks, dependency_data)
    }

    fn is_downwind(face: &Face, direction: &Direction) -> bool {
        let scalar_product = face.normal.dot(&direction.vector);
        scalar_product < OrderedFloat(0.0)
    }

    pub fn from_cell_pairs(cells: Vec<Cell>, pairs: &[(usize, usize)]) -> Grid {
        let edge_list: Vec<(usize, usize, Face)> = pairs
            .iter()
            .map(|(i0, i1)| (*i0, *i1, Grid::face_between(&cells[*i0], &cells[*i1])))
            .collect();
        Grid {
            data: Graph::from_nodes_and_edge_list(cells, edge_list),
        }
    }

    fn face_between(cell_0: &Cell, cell_1: &Cell) -> Face {
        Face {
            normal: (cell_0.center.sub(&cell_1.center)),
        }
    }

    pub fn iter(&self) -> Box<dyn Iterator<Item = &Cell> + '_> {
        self.data.iter()
    }

    pub fn iter_nodes(&self) -> Box<dyn Iterator<Item = &Node<Cell, Face>> + '_> {
        self.data.iter_nodes()
    }

    pub fn get_cell_by_index_mut(&mut self, index: Index) -> &mut Cell {
        &mut self.data.get_mut(index).unwrap().data
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;
    use crate::vector_3d::Vector3D;
    #[test]
    fn dependency_graph() {
        let direction = Direction {
            index: 0,
            vector: Vector3D::new(1.0, 0.0, 0.0),
        };
        let cells = vec![
            Cell {
                global_index: 0,
                local_index: 0,
                center: Vector3D::new(0., 0., 0.),
                processor_num: 0,
            },
            Cell {
                global_index: 1,
                local_index: 0,
                center: Vector3D::new(1., 0., 0.),
                processor_num: 0,
            },
        ];
        let first_cell = cells[0].clone();
        let grid = Grid::from_cell_pairs(cells, &[(0, 1)]);
        let graph = grid.get_dependency_graph(&direction);
        let nodes = graph.traverse_depth_first(&Task {
            cell: &first_cell,
            direction,
            processor_num: 0,
            num_upwind: 0,
        });
        let labels: Vec<Task> = nodes.iter().map(|node| node.data.clone()).collect();
        assert_tasks_equal(&labels, &[(0, 0), (1, 0)]);
    }

    fn assert_tasks_equal(tasks: &[Task], indices: &[(usize, usize)]) {
        for task_info in tasks.iter().zip_longest(indices.iter()) {
            match task_info {
                itertools::EitherOrBoth::Both(task, (cell_index, dir_index)) => {
                    assert_eq!(task.cell.global_index, *cell_index);
                    assert_eq!(task.direction.index, *dir_index);
                }
                _ => {
                    assert!(false);
                }
            };
        }
    }
}
