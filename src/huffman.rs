use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use Tree::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tree<T> {
    Node {
        freqs: u32,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
    Leaf {
        freqs: u32,
        token: T,
    },
}
