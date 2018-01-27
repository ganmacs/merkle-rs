use digestible::{Digestible, Digest, vector_digest, concat_digest};
use token::Token;

#[derive(Clone, Debug)]
pub enum Node {
    Leaf { hash: Digest },
    Inner {
        range: NodeRange,
        left: Box<Node>,
        right: Box<Node>,
    },
}

impl Node {
    pub fn empty_leaf() -> Self {
        Node::Leaf { hash: vector_digest(&vec![]) }
    }

    pub fn new_leaf(value: &Vec<u8>) -> Self {
        Node::Leaf { hash: vector_digest(value) }
    }

    pub fn new_node(range: NodeRange, left: Self, right: Self) -> Self {
        Node::Inner {
            range: range,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn update<T>(&mut self, token: &Token<T>, value: &Vec<u8>) {
        match *self {
            Node::Leaf { .. } => self.update_leaf(value),
            Node::Inner {
                ref range,
                ref mut left,
                ref mut right,
            } => {
                match range.position(&token) {
                    Pos::Left => left.update(token, value),
                    Pos::Right => right.update(token, value),
                    Pos::None => panic!("the value should be in range"),
                }
            }
        }
    }

    pub fn update_leaf(&mut self, value: &Vec<u8>) {
        match *self {
            Node::Leaf { ref mut hash } => *hash = vector_digest(value),
            Node::Inner { .. } => unreachable!(),
        }
    }

    pub fn search_in(&self, search_range: &NodeRange) -> Option<&Node> {
        match *self {
            Node::Leaf { .. } => None,
            Node::Inner {
                ref left,
                ref right,
                ref range,
                ..
            } => {
                match search_range.cover(range) {
                    Cover::Full => Some(self),
                    Cover::None => None,
                    Cover::Partial => {
                        let l = left.search_in(search_range);
                        if l.is_some() {
                            return l;
                        }
                        let r = right.search_in(search_range);
                        if r.is_some() {
                            return r;
                        }
                        None
                    }
                }
            }
        }
    }
}

#[test]
fn test_update_leaf() {
    let mut n1 = Node::empty_leaf();
    let n2 = Node::new_leaf(&vec![1, 2, 3]);

    assert!(n1.digest().as_ref() != n2.digest().as_ref());

    n1.update_leaf(&vec![1, 2, 3]);
    assert!(n1.digest().as_ref() == n2.digest().as_ref());
}

#[test]
fn test_update_node() {
    let n1 = Node::new_node(
        NodeRange::new(1, 2),
        Node::new_leaf(&vec![1, 2, 3]),
        Node::new_leaf(&vec![1, 2, 3]),
    );

    let mut n2 = Node::new_node(
        NodeRange::new(1, 2),
        Node::empty_leaf(),
        Node::new_leaf(&vec![1, 2, 3]),
    );

    assert!(n1.digest().as_ref() != n2.digest().as_ref());

    n2.update(&Token::new(1), &vec![1, 2, 3]);

    assert!(n1.digest().as_ref() == n2.digest().as_ref());
}

impl Digestible for Node {
    fn digest(&self) -> Digest {
        match *self {
            Node::Leaf { hash } => hash,
            Node::Inner {
                ref left,
                ref right,
                ..
            } => concat_digest(&left.digest(), &right.digest()),
        }
    }

    fn is_same_digest<T: Digestible>(&self, other: &T) -> bool {
        self.digest().as_ref() == other.digest().as_ref()
    }
}

#[test]
fn test_digest_value() {
    let n1: Node = Node::new_leaf(&vec![1]);
    let n2: Node = Node::new_leaf(&vec![1]);
    assert!(n1.digest().as_ref() == n2.digest().as_ref());

    let n1: Node = Node::new_leaf(&vec![1]);
    let n2: Node = Node::new_leaf(&vec![2]);
    assert!(n1.digest().as_ref() != n2.digest().as_ref());

    let r = NodeRange::new(1, 2);
    let n3 = Node::new_node(
        r.clone(),
        Node::new_leaf(&vec![1]),
        Node::new_leaf(&vec![1]),
    );
    let n4 = Node::new_node(
        r.clone(),
        Node::new_leaf(&vec![1]),
        Node::new_leaf(&vec![1]),
    );
    assert!(n3.digest().as_ref() == n4.digest().as_ref());
}


#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct NodeRange {
    start: i64,
    end: i64,
    mid: i64,
}

pub enum Pos {
    None,
    Left,
    Right,
}

pub enum Cover {
    Full,
    Partial,
    None,
}

impl NodeRange {
    // u64 is enough?
    pub fn new(start: i64, end: i64) -> Self {
        // absoute value?
        let mid = (start + end) / 2;
        NodeRange { mid, start, end }
    }

    pub fn from_token<T: Clone>(st: Token<T>, et: Token<T>) -> Self {
        NodeRange::new(st.value(), et.value())
    }

    pub fn midpoint(&self) -> Token<i64> {
        Token::new(self.mid)
    }

    pub fn is_contain<T>(&self, token: &Token<T>) -> bool {
        match self.position(&token) {
            Pos::None => false,
            _ => true,
        }
    }

    pub fn partition_by_mid(&self) -> Option<(NodeRange, NodeRange)> {
        if self.size() == 0 {
            None
        } else {
            Some((
                NodeRange::new(self.start, self.mid),
                NodeRange::new(self.mid + 1, self.end),
            ))
        }
    }

    pub fn size(&self) -> usize {
        (self.start.abs() - self.end.abs()).abs() as usize
    }

    pub fn cover(&self, other: &NodeRange) -> Cover {
        if self.size() == 0 {
            if self.start <= other.start || self.end <= other.end {
                Cover::Full
            } else {
                Cover::None
            }
        } else if self.start <= other.start && other.end <= self.end {
            Cover::Full
        } else if self.end < other.start || self.start > other.end {
            Cover::None
        } else {
            Cover::Partial
        }
    }

    pub fn position<T>(&self, token: &Token<T>) -> Pos {
        let v = token.value();

        if self.start <= v && v <= self.end {
            if v <= self.mid { Pos::Left } else { Pos::Right }
        } else {
            Pos::None
        }
    }
}
