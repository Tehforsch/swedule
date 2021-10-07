use crate::cell::Cell;
use crate::direction::Direction;
use crate::task_priority::TaskPriority;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Task<'a> {
    pub cell: &'a Cell,
    pub direction: Direction,
    pub processor_num: usize,
    pub num_upwind: usize,
}

impl<'a> Task<'a> {
    pub fn get_priority(&self) -> TaskPriority {
        TaskPriority {
            priority: self.cell.global_index + self.direction.index * 1000000,
        }
    }
}

impl<'a> std::fmt::Debug for Task<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({},{})@{}",
            self.cell.global_index, self.direction.index, self.processor_num
        )
    }
}
