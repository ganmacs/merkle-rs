use digest::{Digestible, digest};

#[derive(Clone, Debug)]
pub enum Node<T> {
    Leaf { hash: Vec<u8> },
    Inner {
        token: T,
        left: Box<Node<T>>,
        right: Box<Node<T>>,
    },
}

impl<T> Node<T> {
    pub fn empty_leaf() -> Self {
        Node::Leaf { hash: digest(&vec![]) }
    }

    pub fn new_leaf(value: T) -> Self
    where
        T: Digestible,
    {
        Node::Leaf { hash: digest(&value) }
    }

    pub fn new_node(token: T, left: Self, right: Self) -> Self {
        Node::Inner {
            token: token,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn left(&self) -> Option<&Self> {
        match self {
            &Node::Leaf { .. } => None,
            &Node::Inner { ref left, .. } => Some(left),
        }
    }

    pub fn right(&self) -> Option<&Self> {
        match self {
            &Node::Leaf { .. } => None,
            &Node::Inner { ref right, .. } => Some(right),
        }
    }
}
