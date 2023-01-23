use core::fmt::Debug;

#[derive(Debug)]
pub struct Stack<T> {
    top: usize,
    data: Vec<Option<T>>,
}

impl<T: Clone + Debug> Stack<T> {
    pub fn new() -> Self {
        Self {
            top: 0,
            data: vec![None; 5],
        }
    }

    pub fn push(&mut self, item: T) {
        if self.is_full() {
            self.increase_array_size();
        }

        self.data[self.top] = Some(item);
        self.top += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let item = self.data[self.top - 1].clone();
        self.top -= 1;

        return item;
    }

    fn increase_array_size(&mut self) {
        let current_capacity = self.data.len();
        let new_capacity = current_capacity + 5;
        let mut new_data = vec![None; new_capacity];

        for i in 0..current_capacity {
            new_data[i] = self.data[i].clone();
        }

        self.data = new_data;
    }
    
    fn is_full(&self) -> bool {
        return self.top == self.data.len();
    }

    fn is_empty(&self) -> bool {
        return self.top <= 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let stack: Stack<i32> = Stack::new();
        assert_eq!(stack.top, 0);
        assert_eq!(stack.data.len(), 5);
    }

    #[test]
    fn test_push() {
        let mut stack: Stack<i32> = Stack::new();
        
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);
        stack.push(5);
        stack.push(6);

        assert_eq!(stack.top, 6);
        assert_eq!(stack.data.len(), 10);
    }

    #[test]
    fn test_pop() {
        let mut stack: Stack<i32> = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_is_empty() {
        let mut stack: Stack<i32> = Stack::new();

        assert!(stack.is_empty());

        stack.push(1);

        assert!(!stack.is_empty());
    }
}
