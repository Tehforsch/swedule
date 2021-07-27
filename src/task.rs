use crate::{cell::Cell, direction::Direction};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Task<'a, 'b> {
    pub cell: &'a Cell,
    pub direction: &'b Direction,
}
