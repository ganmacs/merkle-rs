use digest::{Digestible, digest, digest2};
use std;

#[derive(Clone, Debug)]
pub enum HashTree<T> {
    Leaf { hash: Vec<u8>, value: T },
    Node {
        hash: Vec<u8>,
        left: Box<HashTree<T>>,
        right: Box<HashTree<T>>,
    },
}

impl<T> PartialEq for HashTree<T> {
    fn eq(&self, other: &Self) -> bool {
        self.hash() == other.hash()
    }
}

impl<T> Digestible for HashTree<T> {
    fn as_ref(&self) -> &[u8] {
        std::convert::AsRef::as_ref(self.hash())
    }
}

impl<T> HashTree<T> {
    pub fn new_leaf(value: T) -> Self
    where
        T: Digestible,
    {
        HashTree::Leaf {
            hash: digest(&value),
            value: value,
        }
    }

    pub fn new_node(left: Self, right: Self) -> Self {
        HashTree::Node {
            hash: digest2(&left, &right),
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn left(&self) -> Option<&HashTree<T>> {
        match self {
            &HashTree::Leaf { .. } => None,
            &HashTree::Node { ref left, .. } => Some(left),
        }
    }

    pub fn right(&self) -> Option<&HashTree<T>> {
        match self {
            &HashTree::Leaf { .. } => None,
            &HashTree::Node { ref right, .. } => Some(right),
        }
    }

    pub fn hash(&self) -> &Vec<u8> {
        match self {
            &HashTree::Leaf { ref hash, .. } => hash,
            &HashTree::Node { ref hash, .. } => hash,
        }
    }
}
