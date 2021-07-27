use generational_arena::Index;

use crate::edge::Edge;

pub struct Node<N, E> {
    pub label: N,
    pub index: Index,
    pub edges: Vec<Edge<E>>,
}
