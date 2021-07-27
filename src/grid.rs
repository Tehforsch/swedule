use crate::{cell::Cell, graph::Graph, task::Task, vector_3d::Vector3D, face::Face};

pub struct Grid {
    cells: Graph<Cell, Face>,
}

impl Grid {
    fn get_dependency_graph(&self, direction: Vector3D) -> Graph<Task, ()> {
        todo!()
    }
}
