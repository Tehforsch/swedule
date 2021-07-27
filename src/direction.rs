use crate::vector_3d::Vector3D;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Direction {
    pub vector: Vector3D,
    pub index: i32,
}
