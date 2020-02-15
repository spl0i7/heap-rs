#[cfg(test)]
mod tests {
    use crate::Heap;
    use crate::HeapKind::{MinHeap, MaxHeap};
    use rand::RngCore;

    #[test]
    fn heap_new() {
        let min_heap: Heap<i64> = Heap::new(MinHeap);
        assert!(min_heap.kind == MinHeap);

        let max_heap: Heap<i64> = Heap::new(MaxHeap);
        assert!(max_heap.kind == MaxHeap);
    }

    #[test]
    fn heap_insert() {
        let mut min_heap: Heap<i64> = Heap::new(MinHeap);
        min_heap.insert(1);
        min_heap.insert(5);
        min_heap.insert(2);
        min_heap.insert(3);

        assert!(min_heap.store == vec![1, 3, 2, 5]);


        let mut max_heap: Heap<i64> = Heap::new(MaxHeap);
        max_heap.insert(1);
        max_heap.insert(5);
        max_heap.insert(2);
        max_heap.insert(3);

        assert!(max_heap.store == vec![5, 3, 2, 1]);
    }

    #[test]
    fn heap_pop() {
        let mut min_heap: Heap<i64> = Heap::new(MinHeap);
        min_heap.insert(1);
        min_heap.insert(5);
        min_heap.insert(2);
        min_heap.insert(3);

        assert_eq!(min_heap.pop().unwrap(), 1);
        assert_eq!(min_heap.pop().unwrap(), 2);
        assert_eq!(min_heap.pop().unwrap(), 3);
        assert_eq!(min_heap.pop().unwrap(), 5);
        assert!(min_heap.pop() == None);


        let mut max_heap: Heap<i64> = Heap::new(MaxHeap);
        max_heap.insert(1);
        max_heap.insert(5);
        max_heap.insert(2);
        max_heap.insert(3);

        assert_eq!(max_heap.pop().unwrap(), 5);
        assert_eq!(max_heap.pop().unwrap(), 3);
        assert_eq!(max_heap.pop().unwrap(), 2);
        assert_eq!(max_heap.pop().unwrap(), 1);
        assert!(max_heap.pop() == None);
    }

    #[test]
    fn random_pop() {
        let mut min_heap = Heap::new(MinHeap);
        let mut r = rand::thread_rng();
        for _ in 1..10000 {
            min_heap.insert(r.next_u32());
        }

        let mut prev = min_heap.pop().unwrap();
        for _ in 1..9999 {
            let current = min_heap.pop();
            assert!(current != None);
            assert!(current.unwrap() >= prev);
            prev = current.unwrap();
        }


        let mut max_heap = Heap::new(MaxHeap);
        let mut r = rand::thread_rng();
        for _ in 1..10000 {
            max_heap.insert(r.next_u32());
        }

        let mut prev = max_heap.pop().unwrap();
        for _ in 1..9999 {
            let current = max_heap.pop();
            assert!(current != None);
            assert!(current.unwrap() <= prev);
            prev = current.unwrap();
        }
    }
}

#[derive(PartialEq)]
pub enum HeapKind {
    MaxHeap,
    MinHeap,
}

pub struct Heap<V: PartialOrd> {
    kind: HeapKind,
    store: Vec<V>,
}

impl<V: PartialOrd> Heap<V> {
    pub fn new(kind: HeapKind) -> Self {
        Heap {
            kind,
            store: Vec::new(),
        }
    }

    pub fn peek(&mut self) -> Option<&V> {
        if self.store.len() > 0 {
            return self.store.first();
        }
        return None;
    }

    fn sift_down(&mut self) {
        let mut current_idx = 0;

        'outer: loop {
            let left_child = current_idx * 2 + 1;
            let right_child = current_idx * 2 + 2;


            match self.kind {
                HeapKind::MaxHeap => {
                    let mut largest = current_idx;
                    if left_child < self.store.len() && self.store[left_child] > self.store[largest] {
                        largest = left_child;
                    }
                    if right_child < self.store.len() && self.store[right_child] > self.store[largest] {
                        largest = right_child;
                    }

                    if largest != current_idx {
                        self.store.swap(largest, current_idx);
                        current_idx = largest;
                    } else {
                        break 'outer;
                    }
                }
                HeapKind::MinHeap => {
                    let mut smallest = current_idx;
                    if left_child < self.store.len() && self.store[left_child] < self.store[smallest] {
                        smallest = left_child;
                    }
                    if right_child < self.store.len() && self.store[right_child] < self.store[smallest] {
                        smallest = right_child;
                    }

                    if smallest != current_idx {
                        self.store.swap(smallest, current_idx);
                        current_idx = smallest;
                    } else {
                        break 'outer;
                    }
                }
            }
        }
    }

    pub fn pop(&mut self) -> Option<V> {
        if self.store.len() > 0 {
            let last_index = self.store.len() - 1;
            self.store.swap(0, last_index);
            let result = self.store.pop();
            self.sift_down();
            return result;
        }
        return None;
    }

    pub fn insert(&mut self, v: V) {
        self.store.push(v);
        self.sift_up();
    }

    fn sift_up(&mut self) {
        let mut current_idx = self.store.len() - 1;

        while current_idx > 0 {
            let parent = (current_idx - 1) / 2;
            match self.kind {
                HeapKind::MaxHeap => {
                    if self.store[current_idx] > self.store[parent] {
                        self.store.swap(current_idx, parent);
                    }
                    current_idx = parent;
                }
                HeapKind::MinHeap => {
                    if self.store[current_idx] < self.store[parent] {
                        self.store.swap(current_idx, parent);
                    }
                    current_idx = parent;
                }
            }
        }
    }
}
