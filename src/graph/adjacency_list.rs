use std::hash::Hash;
// TODO: Use our own hash map.
use std::collections::HashMap;


/// A standard adjacency list implementation of a graph.
///
/// Here the graph is implemented as a `Vec` of nodes, and each node has
/// a vector of edges. Edges contain the index of the node to which they point,
/// in addition to other information.
///
/// For more information read about [adjacency lists][1] on Wikipedia.
///
/// [1]: https://en.wikipedia.org/wiki/Adjacency_list
#[derive(Debug)]
pub struct Graph<K: Hash + Eq, V, E: Copy> {
    nodes: HashMap<K, Box<Node<K, V, E>>>,
}

/// A node containing some value and adjacent nodes.
#[derive(Debug)]
struct Node<K: Hash + Eq, V, E: Copy> {
    value: V,
    adjacencies: Vec<Edge<K, E>>,
}

/// An edge pointing to a node's index.
///
/// Edges also contain a value, which is of some type `E`. This allows for
/// representing weighted graphs or other kinds of data structures.
#[derive(Debug)]
struct Edge<K: Hash + Eq, E: Copy> {
    value: E,
    adjacency: K,
}


impl<K: Hash + Eq, V, E: Copy> Graph<K, V, E> {
    /// Create a new graph with no nodes are edges.
    pub fn new() -> Self {
        Graph { nodes: HashMap::new() }
    }

    /// Add a node to the graph. This node will have the given value, and
    /// no adjacencies.
    pub fn add_node(&mut self, key: K, value: V) {
        self.nodes.insert(key, Box::new(Node::new(value)));
    }

    /// Add an edge to the graph from `a` to `b`.
    pub fn add_edge(&mut self, a: K, b: K, value: E) {
        if self.nodes.contains_key(&a) && self.nodes.contains_key(&b) {
            self.nodes.get_mut(&a).expect("has this key")
                .add_adjacency(Edge::new(value, b));
        }
    }

    /// Add an edge to the graph from `a` to `b`.
    pub fn connect(&mut self, a: K, b: K, value: E)
    where K: Copy {
        if self.nodes.contains_key(&a) && self.nodes.contains_key(&b) {
            self.nodes.get_mut(&a).expect("has this key")
                .add_adjacency(Edge::new(value, b));
            self.nodes.get_mut(&b).expect("has this key")
                .add_adjacency(Edge::new(value, a));
        }
    }
}

impl<K: Hash + Eq, V, E: Copy> Node<K, V, E> {
    /// Create a new node, with no adjacencies.
    fn new(value: V) -> Self {
        Node {
            value: value,
            adjacencies: Vec::new(),
        }
    }

    /// Adds an edge to this node's adjacencies.
    fn add_adjacency(&mut self, adjacency: Edge<K, E>) {
        self.adjacencies.push(adjacency);
    }
}

impl<K: Hash + Eq, E: Copy> Edge<K, E> {
    /// Create a new edge to the given node's index.
    fn new(value: E, adjacency: K) -> Self {
        Edge {
            value: value,
            adjacency: adjacency,
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    // TODO
    #[test]
    fn test_nothing() {
        let mut graph = Graph::new();
        graph.add_node(1, 2);
        graph.add_node(2, 3);
        graph.add_edge(1, 2, "test");
        graph.connect(1, 2, "bi");
        println!("{:?}", graph);
        assert!(false);
    }
}