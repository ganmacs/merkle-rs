use std::ops::Range;
use digest::{Digestible, digest};

use token::Token;
use row_hash::RowHash;

#[derive(Clone, Debug)]
pub enum Node<T> {
    Leaf { hash: Vec<u8> },
    Inner {
        token: T,
        left: Box<Node<T>>,
        right: Box<Node<T>>,
        range: Range<T>,
    },
}

impl<T: Token + PartialOrd> Node<T> {
    pub fn empty_leaf() -> Self {
        Node::Leaf { hash: digest(&vec![]) }
    }

    pub fn new_leaf<V>(value: V) -> Self
    where
        V: Digestible,
    {
        Node::Leaf { hash: digest(&value) }
    }

    pub fn new_node(token: T, range: Range<T>, left: Self, right: Self) -> Self {
        Node::Inner {
            token: token,
            range: range,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn find_and_update<V>(&mut self, rh: &RowHash<T, V>)
    where
        V: Digestible,
    {
        self.inner_find_mut(&rh.key, &rh.value);
    }

    fn inner_find_mut<V>(&mut self, tkn: &T, value: &V)
    where
        V: Digestible,
    {
        match self {
            &mut Node::Leaf { .. } => self.update_hash(value),
            &mut Node::Inner {
                ref range,
                ref mut left,
                ref mut right,
                ref token,
            } => {
                if &range.start <= tkn && tkn <= token {
                    left.inner_find_mut(tkn, value);
                }

                if token < tkn && tkn <= &range.end {
                    right.inner_find_mut(tkn, value);
                }
            }
        }
    }

    pub fn update_hash<V>(&mut self, value: &V)
    where
        V: Digestible,
    {
        // XXX
        match self {
            &mut Node::Leaf { ref mut hash } => *hash = digest(value),
            &mut Node::Inner { .. } => unreachable!(),
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
// impl<T> Iterator for Node<T> {
//     type Item = T;
//     fn next(&mut self) -> Option<Self::Item> {
//         match self {
//             &mut Node::Leaf { .. } => None,
//             &mut Node::Inner { token, right, left } => {}
//         }
//     }
// }
