use std::collections::BinaryHeap;
use crate::search::State;
use std::rc::Rc;

/// Adaptors to create a common interface for different queue implementations, such as FIFO Queue
/// and Priority Queue.

pub trait Queue<S: State> {
    fn enqueue(&mut self, transition: Rc<Transition<S>>);
    fn dequeue(&mut self) -> Option<Rc<Transition<S>>>;
    fn is_empty(&self);
}

pub struct Fifo<S: State> {
    queue: VecDeque<Rc<Transition<S>>>,
}

impl<S: State> Fifo<S> {
    fn new() -> Fifo<S> {
        Fifo { queue:  VecDeque::new() }
    }
}

impl<S: State> Queue<S> for Fifo<S> {
    fn enqueue(&mut self, transition: Rc<Transition<S>>) {
        self.queue.push_back(transition);
    }

    fn dequeue(&mut self) -> Option<Rc<Transition<S>>> {
        self.queue.pop_front()
    }

    fn is_empty(&self) {

    }
}

pub struct Priority<S: State> {
    queue: BinaryHeap<Rc<Transition<S>>>,
}

impl<S: State> Priority<S> {
    fn new() -> Priority<S> {
        Priority { queue:  BinaryHeap::new() }
    }
}

impl<S: State> Queue<S> for Priority<S> {
    fn enqueue(&mut self, transition: Rc<Transition<S>>) {
        self.queue.push(transition);
    }

    fn dequeue(&mut self) -> Option<Rc<Transition<S>>> {
        self.queue.pop()
    }
}
