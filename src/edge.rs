use generational_arena::Index;

pub struct Edge<E> {
    pub label: E,
    pub index: Index,
}
