use std::hash::Hash;

use std::fmt::Debug;
use std::collections::{HashMap, HashSet};

use generational_arena::{Arena, Index};

use crate::edge::Edge;
use crate::node::Node;

pub struct Graph<N, E> {
    arena: Arena<Node<N, E>>,
}

impl<N, E> Graph<N, E> {
    pub fn from_nodes_and_edge_list(nodes: Vec<N>, edges: Vec<(usize, usize, E)>) -> Graph<N, E>
    where
        N: Eq + Hash + Sized + Clone + Debug,
        E: Clone,
    {
        let mut arena = Arena::new();
        let mut label_indices = HashMap::new();
        for (i, label) in nodes.into_iter().enumerate() {
            let index = arena.insert_with(|index| Graph::node_from_index(label, index));
            label_indices.insert(i, index);
        }
        for (index_0, index_1, edge_data) in edges.into_iter() {
            let index_0 = label_indices.get(&index_0).unwrap();
            let index_1 = label_indices.get(&index_1).unwrap();
            arena.get_mut(*index_0).unwrap().edges.push(Edge {
                index: *index_1,
                label: edge_data,
            });
        }
        Graph { arena }
    }

    pub fn node_from_index(label: N, index: Index) -> Node<N, E> {
        Node {
            label,
            index,
            edges: vec![],
        }
    }

    pub fn iter(&self) -> Box<dyn Iterator<Item = &N> + '_> {
        Box::new(self.arena.iter().map(|(_, node)| &node.label))
    }

    fn iter_nodes(&self) -> Box<dyn Iterator<Item = &Node<N, E>> + '_> {
        Box::new(self.arena.iter().map(|(_, node)| node))
    }

    pub fn iter_edges(&self) -> Box<dyn Iterator<Item = (&N, &N, &E)> + '_> {
        let mut edge_data = vec![];
        for node in self.iter_nodes() {
            for edge in node.edges.iter() {
                edge_data.push((
                    &node.label,
                    &self.arena.get(edge.index).unwrap().label,
                    &edge.label,
                ));
            }
        }
        Box::new(edge_data.into_iter())
    }

    pub fn traverse_depth_first(&self, label: &N) -> Vec<&Node<N, E>>
    where
        N: Hash + Eq,
    {
        let node = self.find_node_by_label(label).unwrap();
        let mut result = vec![node];
        for edge in node.edges.iter() {
            result.extend(
                self.traverse_depth_first(&self.arena[edge.index].label)
                    .into_iter(),
            );
        }
        result
    }

    fn find_node_by_label(&self, label: &N) -> Option<&Node<N, E>>
    where
        N: Hash + Eq,
    {
        self.arena
            .iter()
            .map(|(_, node)| node)
            .find(|node| &node.label == label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn depth_first_traversal() {
        let graph =
            from_node_indices(&[(0, 1), (1, 2), (2, 3), (2, 4), (2, 5)]);
        let nodes = graph.traverse_depth_first(&0);
        let labels: Vec<usize> = nodes.iter().map(|node| node.label).collect();
        assert_eq!(labels, vec![0, 1, 2, 3, 4, 5]);
    }

    fn from_node_indices(edges: &[(usize, usize)]) -> Graph<usize, ()> {
        let nodes: HashSet<usize> = edges
            .iter()
            .map(|edge| &edge.0)
            .chain(edges.iter().map(|edge| &edge.1))
            .map(|node| node.clone())
                   .collect();
        let mut nodes: Vec<usize> = nodes.into_iter().collect();
        nodes.sort();
        let edges: Vec<(usize, usize, ())> = edges.iter().map(|edge| (edge.0, edge.1, ())).collect();
        Graph::from_nodes_and_edge_list(
            nodes,
            edges,
        )
    }
}
