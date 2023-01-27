#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
pub struct Node<T: Clone> {
    prev: Option<Box<Node<T>>>,
    next: Option<Box<Node<T>>>,
    key: T,
}

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
pub struct DoublyLinkedList<T: Clone> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
}

impl<T: std::clone::Clone> Node<T> {
    pub fn new(k: T) -> Self {
        Node {
            prev: None,
            next: None,
            key: k,
        }
    }
}

impl<T: std::clone::Clone + std::fmt::Debug> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    /// Insert a new node with key of type `T` at the start of the Doubly Linked List.
    pub fn insert_at_head(&mut self, key: T) {
        // store a new node on the heap
        let mut node = Box::new(Node::new(key));

        match self.head {
            // handles an empty linked list
            None => {
                // sets the `head` and `tail` pointers of the linked list to
                // point to the new node
                self.head = Some(node.clone());
                self.tail = Some(node);
            }

            // handles a not-empty linked list
            Some(ref mut head) => {
                // sets the prev pointer of the current head to point to the new node
                head.prev = Some(node.clone());

                // sets the next pointer of the new node to point to the current head
                node.next = Some(head.clone());
                
                // sets the head pointer of the linked listto point to the new node
                self.head = Some(node);
            }
        }
    }

    /// Insert a new node with key of type `T` at the end of the Doubly Linked List.
    pub fn insert_at_tail(&mut self, key: T) {
        // store a new node on the heap
        let mut node = Box::new(Node::new(key));

        match self.tail {
            // handles an empty linked list
            None => {
                // sets the `head` and `tail` pointers of the linked list to
                // point to the new node
                self.head = Some(node.clone());
                self.tail = Some(node);
            }

            // handles a not-empty linked list
            Some(ref mut tail) => {
                // sets the next pointer of the current tail to point to the new node
                tail.next = Some(node.clone());

                // sets the prev pointer of the new node to point to the current tail
                node.prev = Some(tail.clone());

                // sets the tail pointer of the list to point to the new node
                self.tail = Some(node);
            }
        }
    }

    /// Traverse the doubly linked list starting from the head node until the end of the list
    pub fn traverse(&self) {
        // start with the head (of course)
        let mut current_node = &self.head;

        // iterate through the linked list. the `while let` statement uses
        // pattern matching, which means that the code block will be executed
        // only if the `current_node` pointer is `Some`. otherwise, that is, in case the
        // current_node is None, that will be the case of an empty doubly linked list or
        // the next node points to None (end of list), the while terminates
        while let Some(node) = current_node {
            println!("{:?}", node.key);
            current_node = &node.next;
        }
    }

    //pub fn insert_at_ith(&mut self, _key: T, _index: u32) {}

    //pub fn delete_head(&mut self) -> Option<T> {}

    //pub fn delete_tail(&mut self) -> Option<T> {}

    //pub fn delete_ith(&mut self, index: u32) -> Option<T> {}

    //pub fn search_by_key(&mut self, key: T) -> Option<T> {}

    //pub fn reverse(&mut self) -> DoublyLinkedList<T> {}

    //pub fn length(&self) -> u32 {}
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_node() {
        let node: Node<i32> = Node::new(1);
        assert_eq!(node.prev, None);
        assert_eq!(node.next, None);
        assert_eq!(node.key, 1);
    }

    #[test]
    fn create_linked_list() {
        let list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        assert_eq!(list.tail, None);
        assert_eq!(list.head, None);
    }

    #[test]
    fn insert_at_head_empty() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();

        list.insert_at_head(0);

        // ensures that both the head and tail are the same as the new node when
        // the list is empty at the time of insertion
        assert_eq!(list.head.as_ref().unwrap().key, 0);
        assert_eq!(list.tail.as_ref().unwrap().key, 0);

        // ensures that both prev and next pointers point to None in the case of
        // only one item in the list
        assert!(list.head.as_ref().unwrap().prev.is_none());
        assert!(list.tail.as_ref().unwrap().next.is_none());
    }

    #[test]
    fn insert_at_head_not_empty() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();

        list.insert_at_head(0);
        list.insert_at_head(1);

        assert_eq!(list.head.as_ref().unwrap().key, 1);
        assert_eq!(list.tail.as_ref().unwrap().key, 0);

        // ensures that the prev pointer of a head is None (it'll allway be),
        // even with more than one node in the list
        assert!(list.head.as_ref().unwrap().prev.is_none());

        // ensures that the next and prev pointers of the head node points to the right nodes
        assert_eq!(list.head.as_ref().unwrap().next.as_ref().unwrap().key, 0);
        assert_eq!(list.head.as_ref().unwrap().prev.as_ref(), None);
    }

    #[test]
    fn insert_at_tail_empty() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();

        list.insert_at_tail(0);

        // ensures that both the head and tail are the same as the new node when
        // the list is empty at the time of insertion
        assert_eq!(list.head.as_ref().unwrap().key, 0);
        assert_eq!(list.tail.as_ref().unwrap().key, 0);

        // ensures that both prev and next pointers point to None in the case of
        // only one item in the list
        assert!(list.head.as_ref().unwrap().prev.is_none());
        assert!(list.tail.as_ref().unwrap().next.is_none());
    }

    #[test]
    fn insert_at_tail_not_empty() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();

        list.insert_at_tail(0);
        list.insert_at_tail(1);

        assert_eq!(list.head.as_ref().unwrap().key, 0);
        assert_eq!(list.tail.as_ref().unwrap().key, 1);

        // ensures that the prev pointer of the tail node points to the right previous node
        assert_eq!(list.tail.as_ref().unwrap().prev.as_ref().unwrap().key, 0);
        assert_eq!(list.tail.as_ref().unwrap().next.as_ref(), None);
    }

    #[test]
    fn traverse() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();

        list.insert_at_head(0);
        list.insert_at_head(1);
        list.insert_at_head(2);
        list.insert_at_head(3);

        list.traverse();
    }
}
