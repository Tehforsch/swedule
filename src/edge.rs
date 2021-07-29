use generational_arena::Index;

pub struct Edge<E> {
    pub data: E,
    pub index: Index,
}
