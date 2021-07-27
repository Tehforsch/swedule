use crate::{cell::Cell, direction::Direction};

pub struct Task<'a> {
    cell: &'a Cell,
    direction: Direction,
}
