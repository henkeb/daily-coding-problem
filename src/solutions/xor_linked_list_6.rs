// An XOR linked list is a more memory efficient doubly linked list. Instead of each node holding next and prev fields, it holds a field named both, which is an XOR of the next node and the previous node. Implement an XOR linked list; it has an add(element) which adds the element to the end, and a get(index) which returns the node at index.
//
// If using a language that has no pointers (such as Python), you can assume you have access to get_pointer and dereference_pointer functions that converts between nodes and memory addresses.
//
// Solution
use std::marker::PhantomPinned;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;

struct Node {
    val: usize,
    xor: usize,
    _pin: PhantomPinned,
}

impl Node {
    fn new(val: usize) -> Pin<Box<Self>> {
        let node = Node {
            val,
            xor: 0,
            _pin: PhantomPinned,
        };
        Box::pin(node)
    }
}

impl Deref for Node {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.xor
    }
}

impl DerefMut for Node {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.xor
    }
}

struct XORLinkedList {
    head_reference: Pin<Box<Node>>,
}

impl XORLinkedList {
    fn new(head_reference: Pin<Box<Node>>) -> Self {
        XORLinkedList { head_reference }
    }

    fn insert(&mut self, val: usize) {
        let mut new_node = Node::new(val);
        new_node.deref_mut() = self.head_reference.xor;

        if self.head_reference.xor != 0 {}
    }
}

fn xor<T>(x: &Node, y: &Node) -> usize {
    get_adress(x) ^ get_adress(y)
}

fn get_adress<T>(element: &T) -> usize {
    usize::from_str_radix(format!("{:p}", element).trim_start_matches("0x"), 16).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        // let a = Node {val: 1}
    }
}
