// Given the root to a binary tree, implement serialize(root), which serializes the tree into a string, and deserialize(s), which deserializes the string back into the tree.
//
// For example, given the following Node class
//
// class Node:
//     def __init__(self, val, left=None, right=None):
//         self.val = val
//         self.left = left
//         self.right = right
//
// The following test should pass:
//
// node = Node('root', Node('left', Node('left.left')), Node('right'))
// assert deserialize(serialize(node)).left.left.val == 'left.left'
//
// Solution
// 1. Define Node struct and its fields and create implementation for new
// 2. Use serde to serialize and deserialize the struct
// 3. Write test
//
use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct Node {
    val: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: String, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Node { val, left, right }
    }
}

fn serialize(node: Node) -> String {
    serde_yaml::to_string(&node).unwrap()
}

fn deserialize(s: String) -> Node {
    serde_yaml::from_str(&s).unwrap()
}

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_and_deserialize_tree() {
        let node = Node::new(
            "root".to_string(),
            Some(Box::new(Node::new(
                "left".to_string(),
                Some(Box::new(Node::new("left.left".to_string(), None, None))),
                None,
            ))),
            Some(Box::new(Node::new("right".to_string(), None, None))),
        );
        assert_eq!(
            deserialize(serialize(node)).left.unwrap().left.unwrap().val,
            "left.left"
        );
    }
}
