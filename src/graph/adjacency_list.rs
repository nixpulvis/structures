use std::marker::PhantomData;

#[derive(Debug)]
pub struct Graph<N, E: Copy> {
    nodes: Vec<Box<Node<N, E>>>,
}

impl<N, E: Copy> Graph<N, E> {
    pub fn new(data: N) -> Graph<N, E> {
        let root = Box::new(Node::new(data));
        Graph {
            nodes: vec!(root),
        }
    }

    pub fn add_node(&mut self, data: N) {
        self.nodes.push(Box::new(Node::new(data)));
    }

    pub fn add_edge(&mut self, data: E, a: usize, b: usize, directionality: EdgeType) {
        if a < self.nodes.len() && b < self.nodes.len() {
            match directionality {
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

#[derive(Debug)]
struct Node<N, E: Copy> {
    data: N,
    adjacencies: Vec<Edge<E>>,
}

impl<N, E: Copy> Node<N, E> {
    fn new(data: N) -> Node<N, E> {
        Node {
            data: data,
            adjacencies: Vec::new(),
        }
    }

    fn add_adjacency(&mut self, adjacency: Edge<E>) {
        self.adjacencies.push(adjacency);
    }
}

#[derive(Debug)]
struct Edge<E: Copy> {
    data: E,
    adjacency: usize,
}

impl<E: Copy> Edge<E> {
    fn new(data: E, adjacency: usize) -> Edge<E> {
        Edge {
            data: data,
            adjacency: adjacency,
        }
    }
}

pub enum EdgeType {
    Directional,
    Bidirectional,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_something() {
        let mut graph = Graph::new(1);
        graph.add_node(2);
        graph.add_edge("foo", 0, 1, EdgeType::Directional);
        graph.add_edge("bar", 0, 1, EdgeType::Bidirectional);
        println!("{:?}", graph);
        assert!(false);
    }
}