use std::hash::Hash;

use std::collections::{HashMap, HashSet};

use generational_arena::{Arena, Index};

use crate::edge::Edge;
use crate::node::Node;

pub struct Graph<N, E> {
    arena: Arena<Node<N, E>>,
}

impl<N, E> Graph<N, E> {
    pub fn from_nodes_and_edge_list(nodes: Vec<N>, edges: &[(N, N, E)]) -> Graph<N, E>
    where N: Eq + Hash + Sized,
          E: Copy {
        let mut arena = Arena::new();
        for label in nodes.into_iter() {
            arena.insert_with(|index| Graph::node_from_index(label, index));
        }
        for (label_0, label_1, edge_data) in edges {
            let index_0 = arena.iter().find(|(_, node)| node.label == *label_0).map(|(index, _)| index).unwrap();
            let index_1 = arena.iter().find(|(_, node)| node.label == *label_1).map(|(index, _)| index).unwrap();
            arena.get_mut(index_0).unwrap().neighbours.push(Edge {
                index: index_1,
                label: *edge_data,
            });
        }
        Graph {
            arena
        }
    }

    pub fn from_edge_list(edges: &[(N, N, E)]) -> Graph<N, E>
    where N: Hash + Eq + Copy,
    E: Copy{
        let mut nodes: HashSet<N> = HashSet::new();
        for label in edges.iter().map(|edge| edge.0).chain(edges.iter().map(|edge| edge.1)) {
            nodes.insert(label);
        }
        Graph::from_nodes_and_edge_list(nodes.into_iter().collect(), edges)
    }

    pub fn node_from_index(label: N, index: Index) -> Node<N, E> {
        Node {
            label,
            index,
            neighbours: vec![],
        }
    }

    pub fn traverse_depth_first(&self, label: &N) -> Vec<&Node<N, E>>
    where N: Hash + Eq {
        let node = self.find_node_by_label(label).unwrap();
        let mut result = vec![node];
        for edge in node.neighbours.iter() {
            result.extend(self.traverse_depth_first(&self.arena[edge.index].label).into_iter());
        }
        result
    }

    fn find_node_by_label(&self, label: &N) -> Option<&Node<N, E>> where
    N: Hash + Eq{
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
