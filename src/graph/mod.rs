//! Graphs are a collection of nodes and edges.
//!
//! There are various kinds of graphs and properties about graphs, but at
//! their core, there are some number of nodes connected to some number of
//! other nodes though edges. Edges may have a direction, and/or a value
//! associated with them.
//!
//! # Examples
//!
//! ```
//! use structures::graph::EdgeType;
//! use structures::graph::adjacency_list::Graph;
//!
//! // Easily create a adjacency list graph.
//! let mut graph = Graph::new();
//! graph.add_node(1);
//! graph.add_node(2);
//! graph.add_edge("0 to 1", 0, 1, EdgeType::Directional);
//! ```

/// Graphs represented as nodes with lists of adjacencies.
pub mod adjacency_list;

/// Types of edges, some graphs do not support all types.
pub enum EdgeType {
    Directional,
    Bidirectional,
}
