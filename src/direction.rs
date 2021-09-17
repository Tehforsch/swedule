use std::f64::consts::PI;

use crate::config::{DIRECTION_BINS_1, DIRECTION_BINS_84};
use crate::vector_3d::Vector3D;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Direction {
    pub vector: Vector3D,
    pub index: usize,
}

pub fn get_directions(num: usize) -> Vec<Direction> {
    match num {
        1 => get_directions_from_constant(&DIRECTION_BINS_1),
        84 => get_directions_from_constant(&DIRECTION_BINS_84),
        _ => get_equally_distributed_directions_on_sphere(num)
    }
}

pub fn get_equally_distributed_directions_on_sphere(num_directions: usize) -> Vec<Direction> {
    // Taken from https://www.cmu.edu/biolphys/deserno/pdf/sphere_equi.pdf
    let a = 4.0 * PI / num_directions as f64;
    let d = a.sqrt();
    let m_theta = (PI / d).round() as i32;
    let d_theta = PI / m_theta as f64;
    let d_phi = a / d_theta;
    let mut points = vec![];
    for m in 0..m_theta {
        let theta = PI * (m as f64 + 0.5) / m_theta as f64;
        let m_phi = (2.0 * PI * theta.sin() / d_phi).round() as i32;
        for n in 0..m_phi {
            let phi = 2.0 * PI * n as f64 / m_phi as f64;
            points.push(Vector3D::from_spherical_angles(theta, phi));
        }
    }
    if points.len() != num_directions {
        println!(
            "Could not equally distribute {} points on a sphere - returned {} instead",
            num_directions,
            points.len()
        );
    }
    points
        .into_iter()
        .enumerate()
        .map(|(i, point)| Direction {
            vector: point,
            index: i,
        })
        .collect()
}

pub fn get_directions_from_constant(constant: &[[f64; 3]]) -> Vec<Direction> {
    constant.iter().enumerate().map(|(index, vec)| {
        Direction {
            index,
            vector: Vector3D::new(vec[0], vec[1], vec[2]),
        }
    }).collect()
}
