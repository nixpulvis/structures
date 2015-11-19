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
pub struct Node<K: Hash + Eq, V, E: Copy> {
    value: V,
    adjacencies: Vec<Edge<K, E>>,
}

/// An edge pointing to a node's index.
///
/// Edges also contain a value, which is of some type `E`. This allows for
/// representing weighted graphs or other kinds of data structures.
#[derive(Debug)]
pub struct Edge<K: Hash + Eq, E: Copy> {
    value: E,
    // TODO: Want to save a  reference here.
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

    /// Returns the node of the given key if it is in the graph.
    pub fn get(&self, key: &K) -> Option<&Box<Node<K, V, E>>> {
        self.nodes.get(key)
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

    /// Returns a reference to the list of adjacencies edges for this node.
    pub fn edges<'a>(&'a self) -> &'a Vec<Edge<K, E>> {
        self.adjacencies.as_ref()
    }

    pub fn nodes(&self) {
        self.adjacencies.iter().map(|e| { e. })
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
}