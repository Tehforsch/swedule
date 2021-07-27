use std::hash::Hash;

use std::collections::HashSet;

use generational_arena::{Arena, Index};

use crate::edge::Edge;
use crate::node::Node;

pub struct Graph<N, E> {
    arena: Arena<Node<N, E>>,
}

impl<N, E> Graph<N, E> {
    pub fn from_nodes_and_edge_list(nodes: Vec<N>, edges: &[(N, N, E)]) -> Graph<N, E>
    where
        N: Eq + Hash + Sized,
        E: Clone,
    {
        let mut arena = Arena::new();
        for label in nodes.into_iter() {
            arena.insert_with(|index| Graph::node_from_index(label, index));
        }
        for (label_0, label_1, edge_data) in edges {
            let index_0 = arena
                .iter()
                .find(|(_, node)| node.label == *label_0)
                .map(|(index, _)| index)
                .unwrap();
            let index_1 = arena
                .iter()
                .find(|(_, node)| node.label == *label_1)
                .map(|(index, _)| index)
                .unwrap();
            arena.get_mut(index_0).unwrap().edges.push(Edge {
                index: index_1,
                label: edge_data.clone(),
            });
        }
        Graph { arena }
    }

    pub fn from_edge_list(edges: &[(N, N, E)]) -> Graph<N, E>
    where
        N: Hash + Eq + Clone,
        E: Clone,
    {
        let mut nodes: HashSet<N> = HashSet::new();
        for label in edges
            .iter()
            .map(|edge| edge.0.clone())
            .chain(edges.iter().map(|edge| edge.1.clone()))
        {
            nodes.insert(label);
        }
        Graph::from_nodes_and_edge_list(nodes.into_iter().collect(), edges)
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
            Graph::from_edge_list(&[(1, 2, ()), (2, 3, ()), (3, 4, ()), (3, 5, ()), (3, 6, ())]);
        let nodes = graph.traverse_depth_first(&1);
        let labels: Vec<i32> = nodes.iter().map(|node| node.label).collect();
        assert_eq!(labels, vec![1, 2, 3, 4, 5, 6]);
    }
}
