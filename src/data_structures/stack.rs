pub struct Stack {
    pub data: Vec<i32>,
    pub counter: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self { 
            data: vec![0; 5],
            counter: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.counter == 0;
    }

    pub fn insert(&mut self, item: i32) {
        self.data[self.counter] = item;
        self.counter += 1;
    }
}

#[cfg(test)]
mod test_stack {
    use super::*;

    #[test]
    fn stack_creation() {
        let stack: Stack = Stack::new();
        assert_eq!(stack.data, vec![0; 5]);
        assert_eq!(stack.counter, 0);
    }

    #[test]
    fn stack_empty() {
        let stack: Stack = Stack::new();
        assert_eq!(stack.is_empty(), true);
    }

    #[test]
    fn stack_insert() {
        let mut stack: Stack = Stack::new();
        stack.insert(1);
        assert_eq!(stack.data[0], 1);
    }
}
