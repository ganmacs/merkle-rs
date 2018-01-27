use std::ops::Range;
use std::fmt;
use digest::{Digestible, digest, digest2};

use token::Token;
use row_hash::RowHash;

#[derive(Clone)]
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

    pub fn hash(&self) -> Vec<u8> {
        match *self {
            Node::Leaf { ref hash } => hash.clone(),
            Node::Inner {
                ref left,
                ref right,
                ..
            } => {
                let ref lh = left.hash();
                let ref rh = right.hash();
                digest2(lh, rh)
            }
        }
    }

    pub fn new_node(token: T, range: Range<T>, left: Self, right: Self) -> Self {
        Node::Inner {
            token: token,
            range: range,
            left: Box::new(left),
            right: Box::new(right),
        }
    }


    pub fn is_leaf(&self) -> bool {
        match self {
            &Node::Inner { .. } => false,
            &Node::Leaf { .. } => true,
        }
    }

    pub fn token(&self) -> u64 {
        match self {
            &Node::Leaf { .. } => 100000,
            &Node::Inner { ref token, .. } => token.value(),
        }
    }

    pub fn range(&self) -> Option<&Range<T>> {
        match self {
            &Node::Leaf { .. } => None,
            &Node::Inner { ref range, .. } => Some(range),
        }
    }

    pub fn range_in(&self, sr: &Range<T>) -> Option<&Node<T>> {
        match *self {
            Node::Leaf { .. } => None, // This is invalid. Leaf should have range
            Node::Inner {
                ref left,
                ref right,
                ref range,
                ..
            } => {
                if sr.start == sr.end {
                    if range.start <= sr.start || sr.end <= range.end {
                        return Some(self);
                    } else {
                        return None;
                    }
                } else if sr.start <= range.start && range.end <= sr.end {
                    return Some(self);
                } else if sr.start > range.end || range.start > sr.end {
                    return None;
                }

                let l = left.range_in(sr);
                if l.is_some() {
                    return l;
                }

                let r = right.range_in(sr);
                if r.is_some() {
                    return r;
                }

                None
            }
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

impl<T: Token> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Node::Leaf { ref hash } => write!(f, "Leaf<{:?}>", hash),
            &Node::Inner {
                ref token,
                ref left,
                ref right,
                ref range,
            } => {
                write!(
                    f,
                    "Inner<\n  Token={}\n  {:?}, {:?}\n>",
                    token.value(),
                    left,
                    right
                )
            }

        }
        // write!(f, " {{ x: {}, y: {} }}", self.x, self.y)
    }
}
