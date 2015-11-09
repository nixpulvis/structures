#[macro_use]
extern crate structures;

use structures::list::LinkedList;
use std::iter::FromIterator;

// TODO: Write a macro that inserts a println! after every expression.

fn main() {
    let list: LinkedList<u32> = linked_list![1,2,3];
    println!("{:?}", list.pop());

    let list_a = &LinkedList::from_iter((0..100));
    let list_b = &LinkedList::from_iter((200..300));
    println!("{:?}", list_a.into_iter().collect::<Vec<_>>());
    println!("{:?}", list_a.into_iter().next());
    println!("{:?}", list_a.into_iter().last());
    println!("{:?}", list_a.into_iter().nth(12));
    println!("{:?}", list_a.into_iter().chain(list_b).collect::<Vec<_>>());
    println!("{:?}", list_a.into_iter().zip(list_b).collect::<Vec<_>>());
    println!("{:?}", list_a.into_iter().map(|i| {
        i * i
    }).collect::<Vec<_>>());
    println!("{:?}", list_a.into_iter().filter(|i| {
        (*i % 2) == 0
    }).collect::<Vec<_>>());
    println!("{:?}", list_a.into_iter().filter_map(|i| {
        if i % 4 == 0 { None } else { Some(i * 4) }
    }).collect::<Vec<_>>());
    println!("{:?}", list_a.into_iter().enumerate().collect::<Vec<_>>());
}
