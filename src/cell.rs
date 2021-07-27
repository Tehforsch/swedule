use crate::vector_3d::Vector3D;

#[derive(Hash, PartialEq, Clone, Debug)]
pub struct Cell {
    pub center: Vector3D,
    pub label: i32,
}

impl Eq for Cell {
    fn assert_receiver_is_total_eq(&self) {}
}
