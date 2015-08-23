extern crate structures;

use structures::list::List;
use std::iter::FromIterator;

fn main() {
    let list = List::from_iter((0..100));
    println!("{:?}", list);
}
