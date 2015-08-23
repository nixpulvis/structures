extern crate structures;

use structures::binary_tree::BinaryTree;

fn main() {
    let tree = BinaryTree::new().push(1);
    println!("{:?}", tree);
}
