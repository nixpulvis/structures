use graph::EdgeType;


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
pub struct Graph<N, E: Copy> {
    nodes: Vec<Box<Node<N, E>>>,
}

/// A node containing some data and adjacent nodes.
#[derive(Debug)]
struct Node<N, E: Copy> {
    data: N,
    adjacencies: Vec<Edge<E>>,
}

/// An edge pointing to a node's index.
///
/// Edges also contain data, which is of some type `E`. This allows for
/// representing weighted graphs or other kinds of data structures.
#[derive(Debug)]
struct Edge<E: Copy> {
    data: E,
    adjacency: usize,
}


impl<N, E: Copy> Graph<N, E> {
    /// Create a new graph with no nodes are edges.
    pub fn new() -> Graph<N, E> {
        Graph { nodes: Vec::new() }
    }

    /// Add a node to the graph. This node will have the given data, and
    /// no adjacencies.
    pub fn add_node(&mut self, data: N) {
        self.nodes.push(Box::new(Node::new(data)));
    }

    /// Add an edge to the graph from `a` to `b`.
    ///
    /// If `typ` is `EdgeType::Bidirectional` then an edge from `b` to `a`
    /// is also created.
    pub fn add_edge(&mut self, data: E, a: usize, b: usize, typ: EdgeType) {
        if a < self.nodes.len() && b < self.nodes.len() {
            match typ {
                EdgeType::Directional => {
                    self.nodes[a].add_adjacency(Edge::new(data, b));
                },
                EdgeType::Bidirectional => {
                    self.nodes[a].add_adjacency(Edge::new(data, b));
                    self.nodes[b].add_adjacency(Edge::new(data, a));
                },
            }
        }
    }
}

impl<N, E: Copy> Node<N, E> {
    /// Create a new node, with no adjacencies.
    fn new(data: N) -> Node<N, E> {
        Node {
            data: data,
            adjacencies: Vec::new(),
        }
    }

    /// Adds an edge to this node's adjacencies.
    fn add_adjacency(&mut self, adjacency: Edge<E>) {
        self.adjacencies.push(adjacency);
    }
}

impl<E: Copy> Edge<E> {
    /// Create a new edge to the given node's index.
    fn new(data: E, adjacency: usize) -> Edge<E> {
        Edge {
            data: data,
            adjacency: adjacency,
        }
    }
}


#[cfg(test)]
mod test {
    use graph::EdgeType;
    use super::*;

    // TODO
}