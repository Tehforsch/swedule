use crate::{cell::Cell, direction::Direction};

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Task<'a> {
    pub cell: &'a Cell,
    pub direction: Direction,
    pub processor_num: usize,
    pub num_upwind: usize,
}

impl<'a> std::fmt::Debug for Task<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})@{}", self.cell.index, self.direction.index, self.processor_num)
    }
}
