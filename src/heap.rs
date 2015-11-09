#[derive(Debug)]
pub struct Heap<T: PartialOrd> {
    data: Vec<T>,
}

impl<T: PartialOrd> Heap<T> {
    pub fn new() -> Heap<T> {
        Heap::default()
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
        let index = self.data.len() - 1;
        if index == 0 { return }
        if (self.data.get(index).expect("exists") <
            self.data.get(index / 2).expect("exists")) {
            self.data.swap(index, index / 2);
        } else {
            // Do nothing.
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn peak(&self) -> Option<&T> {
        self.data.first()
    }
}

impl<T: PartialOrd> Default for Heap<T> {
    fn default() -> Self {
        Heap { data: Vec::default() }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    fn heap_push() {
        let mut heap = Heap::new();
        heap.push(5);
        heap.push(10);
        heap.push(3);
        println!("{:?}", heap);
        assert!(false);
    }
}
