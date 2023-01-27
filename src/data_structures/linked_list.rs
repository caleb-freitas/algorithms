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
    length: u32,
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

impl<T: std::clone::Clone + std::fmt::Debug + std::cmp::PartialEq> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            length: 0,
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

                // set new length
                self.length += 1;
            }

            // handles a not-empty linked list
            Some(ref mut head) => {
                // sets the prev pointer of the current head to point to the new node
                head.prev = Some(node.clone());

                // sets the next pointer of the new node to point to the current head
                node.next = Some(head.clone());
                
                // sets the head pointer of the linked listto point to the new node
                self.head = Some(node);

                // set new length
                self.length += 1;
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

                // set new length
                self.length += 1;
            }

            // handles a not-empty linked list
            Some(ref mut tail) => {
                // sets the next pointer of the current tail to point to the new node
                tail.next = Some(node.clone());

                // sets the prev pointer of the new node to point to the current tail
                node.prev = Some(tail.clone());

                // sets the tail pointer of the list to point to the new node
                self.tail = Some(node);

                // set new length
                self.length += 1;
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

    /// Delete the node at head. Can be used several times when using `insert_at_head`
    /// method.
    pub fn delete_head(&mut self) -> Option<T> {
        match self.head {
            // empty list case
            None => None,
            
            // verify the case when list case is greater or equal to 1
            Some(ref mut head) => {
                // clone the value to be deleted (head key) to return later as Option
                let deleted_key = head.key.clone();

                match head.next {
                    // case when list length is equal to 1
                    None => {
                        // both head and tail are equal to None, so the list will be empty
                        self.head = None;
                        self.tail = None;

                        // set the new length
                        self.length -= 1;
                    }
                    
                    // case when list length is greater than 1
                    Some(ref mut next) => {
                        // update the prev (old head) of the next element to None
                        next.prev = None;

                        // set the new head as the next element of the old head
                        self.head = Some(next.clone());

                        // set the new length
                        self.length -= 1;
                    }
                }
                return Some(deleted_key);
            }
        }
    }

    /// Delete the node at tail. Can be used several times when using `insert_at_tail`
    /// method.
    pub fn delete_tail(&mut self) -> Option<T> {
        match self.tail {
            // empty list case
            None => None,

            // non-empty list case
            Some(ref mut tail) => {
                let deleted_key = tail.key.clone();

                // list length >= 1
                match tail.prev {
                    // list length == 1
                    None => {
                        // both head and tail are equal to None, so the list will be empty
                        self.head = None;
                        self.tail = None;

                        // set the new length
                        self.length -= 1;
                    }

                    // list length > 1
                    Some(ref mut prev) => {
                        // update the next pointer (old tail) of the prev pointer to None
                        prev.next = None;

                        // set the new tail as the next element of the old head
                        self.tail = Some(prev.clone());

                        // set the new length
                        self.length -= 1;
                    }
                }
                return Some(deleted_key);
            }
        }
    }

    pub fn search_by_key(&mut self, key: T) -> Option<&Box<Node<T>>> {
        // set current node as head (starting from head, then)
        let mut current_node = &self.head;

        // very similar code to traverse. could reuse the implementation here
        while let Some(ref node) = current_node {
            // check if the current node key is equal to the provided key to be
            // searched for
            if node.key == key {
                // if this is the case, the return the pointer to the node
                return Some(node);
            }
            // keep searching for the next until None
            current_node = &node.next;
        }

        // otherwise return None, that is, the provided key does not exist in the
        // current doubly linked list
        return None;
    }

    //pub fn insert_at_ith(&mut self, _key: T, _index: u32) {}

    //pub fn delete_ith(&mut self, index: u32) -> Option<T> {}

    //pub fn reverse(&mut self) -> DoublyLinkedList<T> {}
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
        assert_eq!(list.length, 0);
    }

    #[test]
    fn insert_at_head_empty() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();

        list.insert_at_head(0);

        assert_eq!(list.length, 1);

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

        assert_eq!(list.length, 2);

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

        assert_eq!(list.length, 1);

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

        assert_eq!(list.length, 2);
        
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

    #[test]
    fn delete_one_at_head() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        list.insert_at_head(0);

        assert_eq!(list.length, 1);

        list.delete_head();

        assert_eq!(list.length, 0);
        assert_eq!(list.head, None);
        assert_eq!(list.tail, None);
    }

    #[test]
    fn delete_one_at_tail() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        list.insert_at_head(0);

        assert_eq!(list.length, 1);

        list.delete_tail();

        assert_eq!(list.length, 0);
        assert_eq!(list.head, None);
        assert_eq!(list.tail, None);
    }

    #[test]
    fn delete_head_empty_list() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        let res = list.delete_head();
        let expected = None;
        assert_eq!(res, expected)
    }
    
    #[test]
    fn delete_tail_empty_list() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        let res = list.delete_tail();
        let expected = None;
        assert_eq!(res, expected)
    }

    #[test]
    fn delete_many_at_head() {

        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();

        list.insert_at_head(0);
        list.insert_at_head(1);
        list.insert_at_head(2);
        list.insert_at_head(3);

        list.delete_head();
        list.delete_head();

        assert_eq!(list.length, 2);

        // ensures that the head and tail pointes had the correct nodes
        assert_eq!(list.head.as_ref().unwrap().key, 1);
        assert_eq!(list.tail.as_ref().unwrap().key, 0);
    }

    #[test]
    fn delete_many_at_tail() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();

        list.insert_at_tail(0);
        list.insert_at_tail(1);
        list.insert_at_tail(2);
        list.insert_at_tail(3);

        list.delete_tail();
        list.delete_tail();

        assert_eq!(list.length, 2);

        // ensures that the head and tail pointes had the correct nodes
        assert_eq!(list.head.as_ref().unwrap().key, 0);
        assert_eq!(list.tail.as_ref().unwrap().key, 1);
    }

    #[test]
    fn searched_key_dont_exit() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();

        list.insert_at_head(0);
        list.insert_at_head(1);
        list.insert_at_head(2);
        list.insert_at_head(3);

        let res = list.search_by_key(4);

        assert_eq!(res, None);
    }
}
