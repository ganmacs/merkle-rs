use digest::{Digestible, digest, digest2};

#[derive(Clone, Debug)]
pub enum Node {
    Leaf { hash: Vec<u8> },
    Inner {
        hash: Vec<u8>,
        left: Box<Node>,
        right: Box<Node>,
    },
}

impl Node {
    pub fn empty_leaf() -> Self {
        Node::Leaf { hash: digest(&vec![0]) }
    }

    pub fn new_leaf<T>(value: T) -> Self
    where
        T: Digestible,
    {
        Node::Leaf { hash: digest(&value) }
    }

    pub fn new_node(left: Self, right: Self) -> Self {
        let hash = {
            let lh = left.hash();
            let rh = right.hash();
            digest2(lh, rh)
        };

        Node::Inner {
            hash: hash,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn left(&self) -> Option<&Node> {
        match self {
            &Node::Leaf { .. } => None,
            &Node::Inner { ref left, .. } => Some(left),
        }
    }

    pub fn right(&self) -> Option<&Node> {
        match self {
            &Node::Leaf { .. } => None,
            &Node::Inner { ref right, .. } => Some(right),
        }
    }

    pub fn hash(&self) -> &Vec<u8> {
        match self {
            &Node::Leaf { ref hash, .. } => hash,
            &Node::Inner { ref hash, .. } => hash,
        }
    }
}
