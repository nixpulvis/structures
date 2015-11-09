use std::rc::{Rc, Weak};
use std::cmp::{Eq, PartialEq};

#[derive(Clone, Debug)]
pub enum DoublyLinkedList<T> {
    Cons(T, Rc<Box<DoublyLinkedList<T>>>, Weak<Box<DoublyLinkedList<T>>>),
    Nil,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList::Nil
    }

    pub fn is_empty(&self) -> bool {
        match *self {
            DoublyLinkedList::Cons(_, _, _) => false,
            DoublyLinkedList::Nil => true,
        }
    }

    pub fn len(&self) -> usize {
        1
    }

    pub fn push(self, item: T) -> DoublyLinkedList<T> {
        let us = Rc::new(Box::new(self));
        match **us {
            DoublyLinkedList::Cons(_, _, p) => {
                DoublyLinkedList::Cons(item, us, p)
            },
            DoublyLinkedList::Nil => {
                DoublyLinkedList::Cons(item, us, Rc::downgrade(&us))
            },
        }
    }

    pub fn pop(self) -> Option<(T, DoublyLinkedList<T>)> {
        match self {
            DoublyLinkedList::Cons(f, n, _) => Some((f, **n)),
            DoublyLinkedList::Nil => None,
        }
    }
}

impl<T: Eq> Eq for DoublyLinkedList<T> {}

impl<T: Eq> PartialEq for DoublyLinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&DoublyLinkedList::Cons(ref fa, ref na, _),
             &DoublyLinkedList::Cons(ref fb, ref nb, _)) => {
                fa == fb && na == nb
            },
            (&DoublyLinkedList::Nil,
             &DoublyLinkedList::Nil) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::DoublyLinkedList;

    #[test]
    fn test_new() {
        let new_list: DoublyLinkedList<u32> = DoublyLinkedList::new();
        let list = DoublyLinkedList::Nil;
        assert_eq!(new_list, list);
    }
}