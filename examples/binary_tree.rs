extern crate structures;

use structures::tree::BinaryTree;

fn main() {
    let tree = BinaryTree::new().push(1);
    println!("{:?}", tree);
}
