use std::ops::Add;

macro_rules! maybe {
    ($expr:expr) => (match $expr {
        ::std::option::Option::Some(val) => val,
        ::std::option::Option::None => return None,
    })
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BinaryTree<T: PartialEq + PartialOrd + Copy> {
    Node(T, Box<BinaryTree<T>>, Box<BinaryTree<T>>),
    Leaf,
}

impl<T: PartialEq + PartialOrd + Copy> BinaryTree<T> {
    pub fn new() -> BinaryTree<T> {
        BinaryTree::Leaf
    }

    pub fn push(self, item: T) -> BinaryTree<T> {
        match self {
            BinaryTree::Node(i, l, r) => {
                if item > i {
                    BinaryTree::Node(i, l, Box::new(r.push(item)))
                } else {
                    BinaryTree::Node(i, Box::new(l.push(item)), r)
                }
            },
            BinaryTree::Leaf => {
                BinaryTree::Node(item, Box::new(self), Box::new(BinaryTree::Leaf))
            }
        }
    }

    // pub fn pop_first(self) -> Option<(T, BinaryTree<T>)> {
    //     match self {
    //         BinaryTree::Node(i, l, r) => {
    //             if l.count() > 0 {
    //                 let (item, popped) = l.pop_first().unwrap();
    //                 Some(item, )
    //             }
    //         },
    //         BinaryTree::Leaf => None
    //     }
    // }

    pub fn remove(self, item: T) -> Option<(T, Self)> {
        match self {
            BinaryTree::Node(i, l, r) => {
                if item == i {
                    Some((i, *l + *r))
                } else if item > i {
                    let (removed, right) = maybe!(r.remove(item));
                    Some((removed, BinaryTree::Node(i, l, Box::new(right))))
                } else {
                    let (removed, left) = maybe!(l.remove(item));
                    Some((removed, BinaryTree::Node(i, Box::new(left), r)))
                }
            },
            BinaryTree::Leaf => None,
        }
    }

    // pub fn flatten(&self) -> Vec<T> {
    //     match *self {
    //         BinaryTree::Node(i, l, r) => {
    //             let mut flat = l.flatten();
    //             flat.push(i);
    //             flat + r.flatten();
    //             flat
    //         },
    //         BinaryTree::Leaf => Vec::new(),
    //     }
    // }
}

impl<T: PartialEq + PartialOrd + Copy> Default for BinaryTree<T> {
    fn default() -> Self {
        BinaryTree::Leaf
    }
}

impl<T: PartialEq + PartialOrd + Copy> Add for BinaryTree<T> {
    type Output = BinaryTree<T>;

    fn add(self, _rhs: BinaryTree<T>) -> BinaryTree<T> {
        self
    }
}

impl<T: PartialEq + PartialOrd + Copy> Iterator for BinaryTree<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::BinaryTree;

    #[test]
    fn BinaryTree_new() {
        let new_tree: BinaryTree<i32> = BinaryTree::new();
        let tree = BinaryTree::Leaf;
        assert_eq!(new_tree, tree);
    }

    #[test]
    fn BinaryTree_push() {
        let push_tree = BinaryTree::new().push(42);
        let tree = BinaryTree::Node(42, Box::new(BinaryTree::Leaf), Box::new(BinaryTree::Leaf));
        assert_eq!(push_tree, tree);
    }

    // #[test]
    // fn BinaryTree_pop_first() {
    //     let (item, pop_first_tree) = BinaryTree::new().push(5).push(1).push(7).pop_first().unwrap();
    //     let tree = BinaryTree::new().push(5).push(7);
    //     assert_eq!(item, 1);
    //     assert_eq!(pop_first_tree, tree);
    // }

    #[test]
    fn BinaryTree_remove() {
        let (item, remove_tree) = BinaryTree::new().push(6).push(2).remove(6).unwrap();
        let tree = BinaryTree::Node(2, Box::new(BinaryTree::Leaf), Box::new(BinaryTree::Leaf));
        assert_eq!(item, 6);
        assert_eq!(remove_tree, tree);
    }

    #[test]
    fn BinaryTree_Default_default() {
        let default_tree: BinaryTree<&str> = BinaryTree::default();
        assert_eq!(default_tree, BinaryTree::Leaf);
    }

    // #[test]
    // fn BinaryTree_Add_add() {
    //     let add_tree = BinaryTree::new().push(2).push(5).push(0) +
    //                    BinaryTree::new().push(3).push(1).push(7);
    //     assert_eq!(add_tree.count(), 6);
    // }
    //
    // #[test]
    // fn BinaryTree_Iterator_count() {
    //     let count_tree = BinaryTree::new().push(12).push(2);
    //     assert_eq!(count_tree.count(), 2);
    // }
    //
    // #[test]
    // fn BinaryTree_Iterator_collect() {
    //     let collect_tree = BinaryTree::new().push(2).push(5).push(0);
    //     assert_eq!(collect_tree.collect::<Vec<i32>>(), [0, 2, 5]);
    // }
}
