use std::hash::Hash;

use std::collections::{HashMap, HashSet};

use generational_arena::{Arena, Index};

use crate::node::Node;

pub struct Graph<T> {
    arena: Arena<Node<T>>,
}

impl<T> Graph<T> {
    pub fn from_edge_list(edges: &[(T, T)]) -> Graph<T>
    where T: Hash + Eq + Copy {
        let mut arena = Arena::new();
        let mut nodes: HashSet<T> = HashSet::new();
        for label in edges.iter().map(|edge| edge.0).chain(edges.iter().map(|edge| edge.1)) {
            nodes.insert(label);
        }
        let mut label_indices: HashMap<T, Index> = HashMap::new();
        for label in nodes.into_iter() {
            let index = arena.insert_with(|index| Graph::node_from_index(label, index));
            label_indices.insert(label, index);
        }
        for edge in edges {
            let index_0 = label_indices[&edge.0];
            let index_1 = label_indices[&edge.1];
            arena.get_mut(index_0).unwrap().neighbours.push(index_1);
        }
        Graph {
            arena
        }
    }

    pub fn node_from_index(label: T, index: Index) -> Node<T> {
        Node {
            label,
            index,
            neighbours: vec![],
        }
    }

    pub fn traverse_depth_first(&self, label: &T) -> Vec<&Node<T>>
    where T: Hash + Eq {
        let node = self.find_node_by_label(label).unwrap();
        let mut result = vec![node];
        for neighbour in node.neighbours.iter() {
            result.extend(self.traverse_depth_first(&self.arena[*neighbour].label).into_iter());
        }
        result
    }

    fn find_node_by_label(&self, label: &T) -> Option<&Node<T>> where
    T: Hash + Eq{
        self.arena.iter().map(|(_, node)| node).find(|node| &node.label == label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn depth_first_traversal() {
        let graph = Graph::from_edge_list(&[
            (1, 2),
            (2, 3),
            (3, 4),
            (3, 5),
            (3, 6),
            ]);
        let nodes = graph.traverse_depth_first(&1);
        let labels: Vec<i32> = nodes.iter().map(|node| node.label).collect();
        assert_eq!(labels, vec![1, 2, 3, 4, 5, 6]);
    }

}
