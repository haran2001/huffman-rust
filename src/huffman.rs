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

#[allow(dead_code)]
impl<T: Clone> Tree<T> {

    pub fn freqs(&self) -> u32 {
        match self {
            Leaf { freqs, .. } => *freqs,
            Node { freqs, .. } => *freqs,
        }
    }

    pub fn token(&self) -> Option<T> {
        match self {
            Leaf { token, .. } => Some(token.clone()),
            Node { .. } => None,
        }
    }

    pub fn left(&self) -> Option<&Tree<T>> {
        match self {
            Leaf { .. } => None,
            Node { left, .. } => Some(left),
        }
    }

    pub fn right(&self) -> Option<&Tree<T>> {
        match self {
            Leaf { .. } => None,
            Node { right, .. } => Some(right),
        }
    }
 
}

