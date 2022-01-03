use std::cmp::Ordering;
use std::collections::VecDeque;
use std::ops::Deref;
use std::rc::Rc;

use binary_heap_plus::BinaryHeap;
use compare::Compare;

/// Adaptors to create a common interface for different queue implementations, such as FIFO Queue
/// and Priority Queue.

pub trait Queue<T>: Sized {
    fn enqueue(&mut self, item: Rc<T>);
    fn dequeue(&mut self) -> Option<Rc<T>>;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn clear(&mut self);
}

// Classic FIFO queue
pub struct Fifo<T> {
    queue: VecDeque<Rc<T>>,
}

impl<T> Fifo<T> {
    pub fn new() -> Fifo<T> {
        Fifo { queue: VecDeque::new() }
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

    fn clear(&mut self) {
        self.queue.clear();
    }
}

//Priority Queue with Ord comparison
pub struct Priority<T: Ord> {
    queue: BinaryHeap<Rc<T>>,
}

impl<T: Ord> Priority<T> {
    pub fn new() -> Priority<T> {
        Priority { queue: BinaryHeap::new() }
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

    fn clear(&mut self) {
        self.queue.clear();
    }
}

//Priority Queue with customisable comparator
pub struct RcFnComparator<F>(F);

pub struct PriorityCmp<T, F>
    where RcFnComparator<F>: Compare<Rc<T>, Rc<T>>,
{
    queue: BinaryHeap<Rc<T>, RcFnComparator<F>>,
}

impl<T, F> Compare<Rc<T>, Rc<T>> for RcFnComparator<F>
    where F: Fn(&T, &T) -> Ordering,
{
    fn compare(&self, l: &Rc<T>, r: &Rc<T>) -> Ordering {
        (self.0)(l.deref(), r.deref())
    }
}

impl<T, F> PriorityCmp<T, F>
    where RcFnComparator<F>: Compare<Rc<T>, Rc<T>>,
{
    pub fn new(cmp: F) -> Self
        where F: Fn(&T, &T) -> Ordering,
    {
        let queue = BinaryHeap::from_vec_cmp(Vec::new(), RcFnComparator(cmp));

        PriorityCmp { queue }
    }
}

impl<T, F> Queue<T> for PriorityCmp<T, F>
    where RcFnComparator<F>: Compare<Rc<T>, Rc<T>>,
{
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

    fn clear(&mut self) {
        self.queue.clear();
    }
}
