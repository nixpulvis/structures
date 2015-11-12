extern crate structures;

use structures::graph::adjacency_list::Graph;

fn main() {
    let mut graph = Graph::new();
    graph.add_node("A", 1337);
    graph.add_node("B", 42);
    graph.add_edge("A", "B", 1);
    let a = graph.get(&"A").unwrap();
    println!("{:?}", a.adjacencies());
}