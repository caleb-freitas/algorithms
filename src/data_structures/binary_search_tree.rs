#![allow(dead_code)]

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Node<T> {
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
    key: T,
}

impl<T> Node<T> {
    pub fn new(key: T) -> Self {
        Self {
            left: None,
            right: None,
            key,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>
}

impl<T: std::cmp::PartialOrd + Clone> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self {
            root: None,
        }
    }

    /// Insert a new node with the key in the binary tree following the rules.
    pub fn insert(&mut self, key: T) {
        let new_node = Box::new(Node::new(key.clone()));

        match &mut self.root {
            // empty BST case
            None => self.root = Some(new_node),

            // non-empty BST case
            Some(node) => {
                // define the current node the root node
                let mut current_node = node;

                // iterate over all nodes that fits the rules
                loop {

                    // check if the key is less than the current_node.left
                    match key < current_node.key {

                        // if true, do the check with the left part
                        true => {
                            match current_node.left.take() {
                                None => {
                                    current_node.left = Some(new_node);
                                    break;
                                },
                                Some(left_node) => *current_node = left_node,
                            }
                        },

                        // if false, do the check with the right part
                        false => {
                            match current_node.right.take() {
                                None => {
                                    current_node.right = Some(new_node);
                                    break;
                                },
                                Some(right_node) => *current_node = right_node,
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_node() {
        let node: Node<i32> = Node::new(1);
        assert_eq!(node.left, None);
        assert_eq!(node.right, None);
        assert_eq!(node.key, 1);
    }

    #[test]
    fn test_create_bts() {
        let bst: BinarySearchTree<i32> = BinarySearchTree::new();
        assert_eq!(bst.root, None);
    }

    #[test]
    fn test_insert_empty() {
        let mut bst: BinarySearchTree<i32> = BinarySearchTree::new();
        bst.insert(1);
        assert_eq!(bst.root.unwrap().key, 1);
    }

    #[test]
    fn test_insert_not_empty() {
        let mut bst: BinarySearchTree<i32> = BinarySearchTree::new();

        // root
        bst.insert(5);
        
        // left node
        bst.insert(2);

        // right node
        bst.insert(6);

        assert_eq!(bst.root.as_ref().unwrap().key, 5);
        assert_eq!(bst.root.as_ref().unwrap().left.as_ref().unwrap().key, 2);
        assert_eq!(bst.root.as_ref().unwrap().right.as_ref().unwrap().key, 6);
    }
}
