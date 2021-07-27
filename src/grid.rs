use crate::{
    cell::Cell, direction::Direction, face::Face, graph::Graph, task::Task, vector_3d::Vector3D,
};
use itertools::Itertools;
use ordered_float::OrderedFloat;

pub struct Grid {
    data: Graph<Cell, Face>,
}

impl Grid {
    fn get_dependency_graph<'b>(&self, direction: &'b Direction) -> Graph<Task<'_, 'b>, ()> {
        let mut dependency_data = vec![];
        for (upwind_cell, downwind_cell, face) in self.data.iter_edges() {
            if Grid::is_downwind(face, &direction) {
                let upwind_task = Task {
                    cell: upwind_cell,
                    direction,
                };
                let downwind_task = Task {
                    cell: downwind_cell,
                    direction,
                };
                dependency_data.push((upwind_task, downwind_task, ()));
            }
        }
        Graph::from_edge_list(&dependency_data)
    }

    fn is_downwind(face: &Face, direction: &Direction) -> bool {
        let scalar_product = face.normal.dot(&direction.vector);
        scalar_product < OrderedFloat(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dependency_graph() {
        let direction = Direction {
            index: 0,
            vector: Vector3D::new(1.0, 0.0, 0.0),
        };
        let cells = vec![
            Cell {
                label: 0,
                center: Vector3D::new(0., 0., 0.),
            },
            Cell {
                label: 1,
                center: Vector3D::new(1., 0., 0.),
            },
        ];
        let first_cell = cells[0].clone();
        let grid = get_grid_from_cell_pairs(cells, &[(0, 1)]);
        let graph = grid.get_dependency_graph(&direction);
        let nodes = graph.traverse_depth_first(&Task {
            cell: &first_cell,
            direction: &direction,
        });
        let labels: Vec<Task> = nodes.iter().map(|node| node.label.clone()).collect();
        assert_tasks_equal(&labels, &[(0, 0), (1, 0)]);
    }

    fn assert_tasks_equal(tasks: &[Task], indices: &[(i32, i32)]) {
        for task_info in tasks.iter().zip_longest(indices.iter()) {
            match task_info {
                itertools::EitherOrBoth::Both(task, (cell_index, dir_index)) => {
                    assert_eq!(task.cell.label, *cell_index);
                    assert_eq!(task.direction.index, *dir_index);
                }
                _ => {
                    assert!(false);
                }
            };
        }
    }

    fn get_grid_from_cell_pairs(cells: Vec<Cell>, pairs: &[(usize, usize)]) -> Grid {
        let edge_list: Vec<(Cell, Cell, Face)> = pairs
            .into_iter()
            .map(|(i0, i1)| {
                (
                    cells[*i0].clone(),
                    cells[*i1].clone(),
                    face_between(&cells[*i0], &cells[*i1]),
                )
            })
            .collect();
        Grid {
            data: Graph::from_edge_list(&edge_list),
        }
    }

    fn face_between(cell_0: &Cell, cell_1: &Cell) -> Face {
        Face {
            normal: (cell_0.center.sub(&cell_1.center)),
        }
    }
}
