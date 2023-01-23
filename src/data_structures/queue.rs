use core::fmt::Debug;

#[derive(Debug)]
pub struct Queue<T> {
    head: usize,
    tail: usize,
    data: Vec<Option<T>>,
}

impl<T: Clone + Debug> Queue<T> {
    pub fn new() -> Self {
        Self {
            head: 0,
            tail: 0,
            data: vec![None; 5],
        }
    }

    pub fn enqueue(&mut self, item: T) {
        if self.is_full() {
            self.grow();
        }

        self.data[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.data.len();
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let item = self.data[self.head].clone();
        self.head = (self.head + 1) % self.data.len();

        return item;
    }

    fn grow(&mut self) {
        let current_capacity = self.data.len();
        let mut new_data = vec![None; current_capacity + 5];

        for i in 0..current_capacity {
            new_data[i] = self.data[(self.head + i) % current_capacity].take();
        }

        self.head = 0;
        self.tail = self.data.len() - 1;
        self.data = new_data;
    }

    fn is_full(&self) -> bool {
        return (self.tail + 1) % self.data.len() == self.head;
    }

    fn is_empty(&self) -> bool {
        return self.head == self.tail;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let queue: Queue<i32> = Queue::new();
        assert_eq!(queue.data.len(), 5);
    }

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue: Queue<i32> = Queue::new();

        queue.enqueue(1);
        queue.enqueue(2);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_is_empty() {
        let mut queue: Queue<i32> = Queue::new();

        assert!(queue.is_empty());

        queue.enqueue(1);

        assert!(!queue.is_empty());
    }

    #[test]
    fn test_enqueue_grow_when_full() {
        let mut queue: Queue<i32> = Queue::new();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        queue.enqueue(4);
        queue.enqueue(5);

        assert_eq!(queue.data.len(), 10);
    }
}
