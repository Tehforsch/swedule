use generational_arena::Index;

use crate::edge::Edge;

pub struct Node<N, E> {
    pub data: N,
    pub index: Index,
    pub edges: Vec<Edge<E>>,
}
