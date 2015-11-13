extern crate structures;

use structures::graph::adjacency_list::Graph;

fn main() {
    let mut graph = Graph::new();
    graph.push("ryan", 71);
    graph.push("ashley", 62);
    graph.push("ben", 73);
    graph.push("nate", 71);
    graph.connect("ryan", "ashley", 160);
    graph.connect("ryan", "ben", 40);
    graph.connect("ryan", "nate", 20);
    graph.connect("ashley", "ben", 35);
    graph.connect("ashley", "nate", 22);
    graph.connect("ben", "nate", 20);

    println!("{:#?}", graph.get("ryan"));
}