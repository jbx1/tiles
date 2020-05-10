use std::collections::{BinaryHeap, VecDeque};
use std::rc::Rc;

/// Adaptors to create a common interface for different queue implementations, such as FIFO Queue
/// and Priority Queue.

pub trait Queue<T>: Sized {
    fn enqueue(&mut self, item: Rc<T>);
    fn dequeue(&mut self) -> Option<Rc<T>>;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
}

pub struct Fifo<T> {
    queue: VecDeque<Rc<T>>,
}

impl<T> Fifo<T> {
    pub fn new() -> Fifo<T> {
        Fifo { queue:  VecDeque::new() }
    }
}

impl<T> Queue<T> for Fifo<T> {
    fn enqueue(&mut self, item: Rc<T>) {
        self.queue.push_back(item);
    }

    fn dequeue(&mut self) -> Option<Rc<T>> {
        self.queue.pop_front()
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn len(&self) -> usize {
        self.queue.len()
    }
}


pub struct Priority<T: Ord> {
    queue: BinaryHeap<Rc<T>>,
}

//todo: switch to use binary_heap_plus to be able to specify comparator
impl<T: Ord> Priority<T> {
    pub fn new() -> Priority<T> {
        Priority { queue:  BinaryHeap::new() }
    }
}

impl<T: Ord> Queue<T> for Priority<T> {
    fn enqueue(&mut self, item: Rc<T>) {
        self.queue.push(item);
    }

    fn dequeue(&mut self) -> Option<Rc<T>> {
        self.queue.pop()
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn len(&self) -> usize {
        self.queue.len()
    }
}
