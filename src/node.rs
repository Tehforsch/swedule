use generational_arena::Index;

pub struct Node<T> {
    pub label: T,
    pub index: Index,
    pub neighbours: Vec<Index>,
}
