use crate::{cell::Cell, direction::Direction};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Task<'a> {
    pub cell: &'a Cell,
    pub direction: Direction,
    pub processor_num: usize,
    pub num_upwind: usize,
}
