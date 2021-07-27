use crate::vector_3d::Vector3D;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Face {
    pub normal: Vector3D,
}
