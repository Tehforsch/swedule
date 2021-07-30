use ordered_float::OrderedFloat;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Vector3D {
    pub x: OrderedFloat<f64>,
    pub y: OrderedFloat<f64>,
    pub z: OrderedFloat<f64>,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D {
            x: OrderedFloat(x),
            y: OrderedFloat(y),
            z: OrderedFloat(z),
        }
    }

    pub fn sub(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn dot(&self, other: &Vector3D) -> OrderedFloat<f64> {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}
