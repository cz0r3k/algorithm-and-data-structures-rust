use super::PriorityQueue;
pub const MAX_HEAP_SIZE: usize = 128;

#[derive(Debug)]
pub struct Heap {
    last_element: usize,
    heap: [i32; MAX_HEAP_SIZE],
}

impl PriorityQueue for Heap {
    fn new() -> Self {
        let mut h = Heap {
            last_element: 0,
            heap: [0; MAX_HEAP_SIZE],
        };
        h.heap[0] = i32::MAX;
        h
    }
    fn from(values: &[i32]) -> Result<Box<Self>, &'static str> {
        if values.len() >= MAX_HEAP_SIZE {
            Err("Slice too long")
        } else {
            let mut h = Heap::new();
            h.last_element = values.len();
            for i in 1..=h.last_element {
                h.heap[i] = values[i - 1];
            }
            for i in (1..=(h.last_element / 2)).rev() {
                let _res = h.down_heap(i);
            }
            Ok(Box::new(h))
        }
    }
    fn get_max(&self) -> Result<i32, &'static str> {
        if self.is_empty() {
            Err("Empty heap")
        } else {
            Ok(self.heap[1])
        }
    }

    fn insert(&mut self, value: i32) -> Result<(), &'static str> {
        if self.is_full() {
            Err("Full heap")
        } else {
            self.last_element += 1;
            let last_element = self.last_element;
            self.heap[last_element] = value;
            self.up_heap(last_element);
            Ok(())
        }
    }

    fn delete_max(&mut self) -> Result<i32, &'static str> {
        let max = self.get_max();
        if !self.is_empty() {
            self.heap[1] = self.heap[self.last_element];
            self.last_element -= 1;
            self.down_heap(1);
        }
        max
    }
}

impl Heap {
    pub fn size(&self) -> usize {
        self.last_element
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn is_full(&self) -> bool {
        self.size() + 1 == MAX_HEAP_SIZE
    }
    fn down_heap(&mut self, pointer: usize) {
        let mut pointer = pointer;
        while pointer * 2 <= self.last_element {
            let mut child_pointer = pointer * 2;
            if child_pointer < self.last_element
                && self.heap[child_pointer + 1] > self.heap[child_pointer]
            {
                child_pointer += 1;
            }
            if self.heap[child_pointer] > self.heap[pointer] {
                self.heap.swap(child_pointer, pointer);
                pointer = child_pointer;
            } else {
                break;
            }
        }
    }

    fn up_heap(&mut self, pointer: usize) {
        let mut pointer = pointer;
        while self.heap[pointer / 2] < self.heap[pointer] {
            self.heap.swap(pointer / 2, pointer);
            pointer /= 2;
        }
    }
}
