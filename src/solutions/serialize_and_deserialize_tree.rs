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
// 1. Create Node struct
// 2. Create serialize function
// 3. Create deserialize function
// 4. Create test
//

#[allow(dead_code)]
#[derive(Default)]
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

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_and_deserialize_tree() {}
}
