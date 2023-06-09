// A unival tree (which stands for "universal value") is a tree where all nodes under it have the same value.
//
// Given the root to a binary tree, count the number of unival subtrees.
//
// For example, the following tree has 5 unival subtrees:
//
//    0
//   / \
//  1   0
//     / \
//    1   0
//   / \
//  1   1
use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
struct BinaryTree<T> {
    value: T,
    left: Option<Box<BinaryTree<T>>>,
    right: Option<Box<BinaryTree<T>>>,
}

impl<T: PartialEq> BinaryTree<T> {
    pub fn new(value: T) -> Self {
        BinaryTree {
            value,
            left: None,
            right: None,
        }
    }

    pub fn left(mut self, node: BinaryTree<T>) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    pub fn right(mut self, node: BinaryTree<T>) -> Self {
        self.right = Some(Box::new(node));
        self
    }

    pub fn insert(&mut self, new_value: T) {
        let mut queue: VecDeque<&mut BinaryTree<T>> = VecDeque::new();
        queue.push_front(self);

        loop {
            let BinaryTree {
                ref mut left,
                ref mut right,
                ..
            } = queue.pop_back().unwrap();

            match left {
                Some(node) => queue.push_front(node),
                None => {
                    *left = Some(Box::new(BinaryTree::new(new_value)));
                    return;
                }
            }

            match right {
                Some(node) => queue.push_front(node),
                None => {
                    *right = Some(Box::new(BinaryTree::new(new_value)));
                    return;
                }
            }
        }
    }

    pub fn number_of_unival_subtrees(&self) -> usize {
        // BFS traversal
        let mut bfs: VecDeque<&BinaryTree<T>> = VecDeque::new();
        bfs.push_front(self);

        let mut count = 0;

        while !bfs.is_empty() {
            let BinaryTree {
                ref left,
                ref right,
                ref value,
            } = bfs.pop_back().unwrap();

            match left {
                Some(node) => bfs.push_front(node),
                None => (),
            }

            match right {
                Some(node) => bfs.push_front(node),
                None => (),
            }

            match (left, right) {
                (Some(l), Some(r)) => {
                    if (l.value == r.value) && (r.value == *value) {
                        count += 1;
                    }
                }
                (None, None) => count += 1,
                (_, _) => (),
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_tree() {
        let root = BinaryTree::new(5);
        assert_eq!(root.value, 5);
    }

    #[test]
    fn insert_left() {
        let root = BinaryTree::new(15).left(BinaryTree::new(8));
        if let Some(node) = root.left {
            assert_eq!(node.value, 8);
        } else {
            panic!();
        }

        assert_eq!(root.right, None);
    }

    #[test]
    fn insert_right() {
        let root = BinaryTree::new(15).right(BinaryTree::new(8));
        if let Some(node) = root.right {
            assert_eq!(node.value, 8);
        } else {
            panic!();
        }

        assert_eq!(root.left, None);
    }

    #[test]
    fn insertion() {
        let mut root = BinaryTree::new(1);
        root.insert(2);
        root.insert(3);

        assert_eq!(
            root,
            BinaryTree::new(1)
                .left(BinaryTree::new(2))
                .right(BinaryTree::new(3))
        )
    }

    #[test]
    fn unival_subtrees() {
        let tree = BinaryTree::new(0).left(BinaryTree::new(1)).right(
            BinaryTree::new(0).right(BinaryTree::new(0)).left(
                BinaryTree::new(1)
                    .left(BinaryTree::new(1))
                    .right(BinaryTree::new(1)),
            ),
        );

        assert_eq!(tree.number_of_unival_subtrees(), 5);
    }
}
