use crate::vector_3d::Vector3D;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Direction {
    pub vector: Vector3D,
    pub index: usize,
}

fn get_equally_distributed_directions_on_sphere(num_directions: usize) -> Vec<Direction> {
    todo!()
}
