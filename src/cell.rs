use std::str::FromStr;

use regex::Regex;

use crate::vector_3d::Vector3D;

#[derive(Hash, PartialEq, Clone, Debug)]
pub struct Cell {
    pub center: Vector3D,
    pub global_index: usize,
    pub local_index: usize,
    pub processor_num: usize,
}

impl Cell {
    pub fn get_id(&self) -> CellId {
        CellId {
            index: self.local_index,
            processor_num: self.processor_num,
        }
    }
}

impl Eq for Cell {
    fn assert_receiver_is_total_eq(&self) {}
}

#[derive(Hash, PartialEq, Clone, Debug)]
pub struct CellId {
    index: usize,
    processor_num: usize,
}

impl Eq for CellId {
    fn assert_receiver_is_total_eq(&self) {}
}

impl FromStr for CellId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new("([0-9]*),([0-9]*)").unwrap();
        let cap = re.captures_iter(s).next().unwrap();
        Ok(CellId {
            processor_num: cap[1].parse()?,
            index: cap[2].parse()?,
        })
    }
}
