//! Lists are ordered sequences of values, which can be iterated over.
//!
//! Within the context of a type system lists may be all of one type, or of
//! many types. This library will eventually have implementations for all of
//! these types.
//!
//! All lists implement the standard collection traits, `FromIterator`, and
//! `IntoIterator`. This allows lists to be to be both easily converted to
//! and from. Additionally all lists implement the traits `Clone`, `Debug`,
//! `PartialEq`, `Eq`, `Hash`, `PartialOrd`, `Ord`.
//!
//! # Examples
//!
//! ```
//! use std::iter::FromIterator;
//! use structures::list::List;
//!
//! // Easily create a linked list.
//! let list = List::from_iter((0..100));
//! assert_eq!(list.len(), 100);
//! ```

use std::mem;
use std::iter::FromIterator;

/// A List<T> is conceptually one of: `Cost<T, List<T>>`, or `Nil`.
///
/// This implementation of a linked list is purely for the purpose of learning,
/// and as reference of the details of a recursive type. You probably shouldn't
/// use this data structure in real world applications.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

/// Iterator for lists by reference.
pub struct Iter<'a, T: 'a> {
    current: &'a List<T>,
}

/// Iterator for lists by value.
pub struct IntoIter<T> {
    current: List<T>,
}

impl<'a, T> List<T> {
    /// Return a new empty linked list. This is semantically equivlent to
    /// writing `List::Nil`.
    ///
    /// # Examples
    ///
    /// ```
    /// use structures::list::List;
    ///
    /// assert_eq!(List::new(), List::Nil::<u32>);
    /// ```
    pub fn new() -> List<T> {
        List::Nil
    }

    /// Determine if a linked list is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use structures::list::List;
    ///
    /// assert!(List::Nil::<u32>.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        match *self {
            List::Cons(_, _) => false,
            List::Nil => true,
        }
    }

    /// Return the length of a list. This function relies on the implementation
    /// of `into_iter()` for creating a iterator over references to the
    /// elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use structures::list::List;
    ///
    /// assert_eq!(List::new().push(1).len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.into_iter().count()
    }

    /// Add an element to the front of the list. Insert is constant time
    /// because we return the new list containing the item, and the old now
    /// moved list.
    ///
    /// # Examples
    ///
    /// ```
    /// use structures::list::List;
    ///
    /// let list_a = List::new().push(2).push(1);
    /// let list_b = list_a.push(0);
    ///
    /// assert_eq!(list_b.len(), 3);
    ///
    /// // Compiler error: use of moved value: `list_a`
    /// // assert_eq!(list_a.len(), 3);
    /// ```
    pub fn push(self, item: T) -> List<T> {
        List::Cons(item, Box::new(self))
    }

    /// Return both the first and rest of a list, or `None` if called on `Nil`.
    /// Here `Nil` represents the end of the list, and contains no item. The
    /// returned values are moved, so it's important store them if they are
    /// needed later.
    ///
    /// # Examples
    ///
    /// ```
    /// use structures::list::List;
    ///
    /// let list = List::new().push("bye").push("hi");
    /// let (first, rest) = list.pop().unwrap();
    ///
    /// assert_eq!(first, "hi");
    /// assert_eq!(rest, List::new().push("bye"));
    ///
    /// // Compiler error: use of moved value: `list`
    /// // assert_eq!(list.len(), 2);
    /// ```
    pub fn pop(self) -> Option<(T, List<T>)> {
        match self {
            List::Cons(item, rest) => Some((item, *rest)),
            List::Nil => None,
        }
    }

    /// Returns the list with an item inserted at the given index. Indexing
    /// starts at zero. For example `(Cons 7 (Cons 9 (Cons 3)))` has `7` at
    /// index 0, `9` at 1, and `3` at 2. Elements in the list after the
    /// inserted item will all have an index one larger than before, but the
    /// order of these elements is maintained.
    ///
    /// This function returns a `Result` to handle the case when `index` is
    /// out of bounds. In this case, the original list is returned in the
    /// `Err` to allow for it's continued use.
    ///
    /// # Examples
    ///
    /// ```
    /// use structures::list::List;
    ///
    /// let list_a = List::new().push(0)
    ///                               .push(1)
    ///                               .push(2)
    ///                               .insert(1, 42)
    ///                               .unwrap();
    /// let list_b = List::new().push(0)
    ///                               .push(1)
    ///                               .push(42)
    ///                               .push(2);
    ///
    /// assert_eq!(list_a, list_b);
    /// ```
    pub fn insert(self, index: usize, item: T) -> Result<List<T>, List<T>> {
        if index == 0 {
            Ok(self.push(item))
        } else {
            match self {
                List::Cons(i, r) => {
                    match r.insert(index - 1, item) {
                        Ok(l) => Ok(List::Cons(i, Box::new(l))),
                        Err(l) => Err(List::Cons(i, Box::new(l))),
                    }
                },
                List::Nil => {
                    Err(self)
                },
            }
        }
    }

    /// Removes an item by index in the list. Like `pop`, both the item and
    /// the rest of the list are moved and returned. Indexing starts at 0, see
    /// `insert` for an example of the indexing. Like `insert` the order of
    /// persisted elements is unchanged.
    ///
    /// This function returns a `Result` to handle the case when `index` is
    /// out of bounds. In this case, the original list is returned in the
    /// `Err` to allow for it's continued use.
    ///
    /// # Examples
    ///
    /// ```
    /// use structures::list::List;
    ///
    /// let (item, list) = List::new().push(1).push(2).push(3).remove(2).unwrap();
    ///
    /// assert_eq!(item, 1);
    /// assert_eq!(list, List::new().push(2).push(3))
    /// ```
    pub fn remove(self, index: usize) -> Result<(T, List<T>), List<T>> {
        match self {
            List::Cons(i, r) => {
                if index == 0 {
                    Ok((i, *r))
                } else {
                    match r.remove(index - 1) {
                        Ok((j, l)) => Ok((j, List::Cons(i, Box::new(l)))),
                        Err(l) => Err(List::Cons(i, Box::new(l))),
                    }
                }
            },
            List::Nil => {
                Err(self)
            },
        }
    }
}

/// This trait allows for creation of a `List<T>` from any type that
/// implements `IntoIterator<Item=T>`. The beauty here is that this essentially
/// allows us to make a `List` from anything that is iterable, without
/// needing to handle different type specially.
///
/// # Examples
///
/// ```
/// use std::iter::FromIterator;
/// use structures::list::List;
///
/// let list_a = List::from_iter((0..10));
/// let list_b = List::from_iter(vec![0,1,2,3,4,5,6,7,8,9]);
///
/// assert_eq!(list_a, list_b);
/// ```
impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iterable: I) -> List<T> {
        iterable.into_iter().fold(List::new(), List::push)
    }
}

/// This trait, implemented for a reference to a `List` allows, that
/// reference to be treated as an iterator by calling `into_iter()` on it.
/// This effectively allows a reference to a linked list to be used as an
/// iterator over type `T` anywhere that accepts `IntoIterator<Item=&'a T>`.
///
/// This trait implementation yields `Iter`s which iterate over references
/// without moving data.
///
/// # Examples
///
/// ```
/// use structures::list::List;
///
/// let list = List::new().push("hello")
///                             .push(" ")
///                             .push("world")
///                             .push("\n");
///
/// for str in &list {
///     print!("{}", str);
/// }
///
/// assert_eq!(list.len(), 4);
/// ```
impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        Iter { current: self }
    }
}

/// This trait, implemented for a reference to a `List` allows, that
/// reference to be treated as an iterator by calling `into_iter()` on it.
/// This effectively allows a reference to a linked list to be used as an
/// iterator over type `T` anywhere that accepts `IntoIterator<Item=&'a T>`.
///
/// This trait implementation yields `IntoIter`s which iterate over moved data.
///
/// # Examples
///
/// ```
/// use structures::list::List;
///
/// let list = List::new().push("hello")
///                             .push(" ")
///                             .push("world")
///                             .push("\n");
///
/// for str in list {
///     print!("{}", str);
/// }
///
/// // Compiler error: use of moved value: `list`.
/// // assert_eq!(list.len(), 4);
/// ```
impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter { current: self }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let current = mem::replace(&mut self.current, List::Nil);
        current.pop().map(|(item, rest)| {
            self.current = rest;
            item
        })
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.current {
            &List::Cons(ref val, ref next) => {
                self.current = next;
                Some(val)
            },
            &List::Nil => {
                None
            },
        }
    }
}

#[cfg(test)]
mod test {
    use std::iter::FromIterator;
    use super::List;

    #[test]
    fn List_new() {
        let new_list: List<u32> = List::new();
        let list = List::Nil;
        assert_eq!(new_list, list);
    }

    #[test]
    fn List_is_empty() {
        let is_empty_list = List::new();
        assert!(is_empty_list.is_empty());
        let is_empty_list = is_empty_list.push(1);
        assert!(!is_empty_list.is_empty());
    }

    #[test]
    fn List_len() {
        let len_list = List::new().push(1).push(2).push(3);
        assert_eq!(len_list.len(), 3);
    }

    #[test]
    fn List_push() {
        let push_list = List::new().push(1).push(2);
        let list = List::Cons(2, Box::new(List::Cons(1, Box::new(List::Nil))));
        assert_eq!(push_list, list);
    }

    #[test]
    fn List_pop() {
        let (item, pop_list) = List::new().push(1).push(2).pop().unwrap();
        let list = List::Cons(1, Box::new(List::Nil));
        assert_eq!(item, 2);
        assert_eq!(pop_list, list);
    }

    #[test]
    fn List_insert_in_bounds() {
        let list = List::new().push(4).push(3).push(1).insert(1, 2).unwrap();
        assert_eq!(Vec::from_iter(list), vec![1, 2, 3, 4]);
    }

    #[test]
    fn List_insert_at_bounds() {
        let list = List::new().push(2).insert(1, 12).unwrap();
        assert_eq!(list, List::new().push(12).push(2));
    }

    #[test]
    fn List_insert_out_of_bounds() {
        let list = List::new().push(2).insert(2, 12).unwrap_err();
        assert_eq!(list, List::new().push(2));
    }

    #[test]
    fn List_remove_in_bounds() {
        let (item, list) = List::new().push(4).push(3).push(2)
                                      .push(1).remove(1).unwrap();
        assert_eq!(item, 2);
        assert_eq!(Vec::from_iter(list), vec![1, 3, 4]);
    }

    #[test]
    fn List_remove_at_bounds() {
        let (item, list) = List::new().push(4).push(3).push(2)
                                      .push(1).remove(3).unwrap();
        assert_eq!(item, 4);
        assert_eq!(Vec::from_iter(list), vec![1, 2, 3]);
    }

    #[test]
    fn List_remove_out_of_bounds() {
        let list = List::new().push(2).remove(1).unwrap_err();
        assert_eq!(list, List::new().push(2));
    }

    #[test]
    fn List_append() {}

    #[test]
    fn Iter() {
        let list: List<u32> = List::new().push(1).push(2).push(3);
        for i in &list {
            assert!((1 <= *i) && (*i <= 3));
        }
    }

    #[test]
    fn IntoIter() {
        let list: List<u32> = List::new().push(1).push(2).push(3);
        assert_eq!(Vec::from_iter(list), vec![3, 2, 1]);
    }
}
