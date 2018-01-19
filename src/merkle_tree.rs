use std::ops::Range;

use token::{IntegerToken, Token};
use digest::Digestible;
use row_hash::RowHash;
use node::Node;
use partitioner::{IPartitioner, Partitioner};

#[derive(Clone, Debug)]
pub struct MerkleTree<T> {
    root: Option<Node<T>>,
    count: usize,
    depth: usize,
    range: Range<T>,
}

impl<T> MerkleTree<T> {
    pub fn new<V, S>(range: Range<T>, v: Vec<RowHash<T, V>>, depth: f64) -> Self
    where
        T: Token<S>,
        V: Digestible,
    {
        let size = depth.log2() as usize;

        MerkleTree {
            root: None,
            count: v.len(),
            depth: depth as usize,
            range: range,
        }
    }

    pub fn build<P, S>(mut self, partitioner: &P)
    where
        P: Partitioner<T, S>,
        T: Token<S>,
    {
        self.root = Some(self.build_node(&self.range, 0, partitioner));
        let v = self.build_node(&self.range, 0, partitioner);
    }

    pub fn build_node<P, S>(&self, range: &Range<T>, depth: usize, partitioner: &P) -> Node<T>
    where
        T: Token<S>,
        P: Partitioner<T, S>,
    {
        match partitioner.call(&range) {
            None => Node::empty_leaf(),
            Some((l, r)) => {
                // left: left <= X <= mid
                let ll = self.build_node(&l, depth + 1, partitioner);
                // right: mid < X <= right
                let rr = self.build_node(&r, depth + 1, partitioner);
                Node::new_node(l.end, ll, rr)
            }
        }
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

#[test]
fn test_height_of_tree() {
    let v = IntegerToken::new(1);
    let v2 = IntegerToken::new(2);
    let v3 = IntegerToken::new(3);
    let v = vec![
        RowHash::new(v, vec![1, 2, 3]),
        RowHash::new(v2, vec![2, 3, 4]),
        RowHash::new(v3, vec![4, 5, 6]),
    ];

    let r = Range {
        start: IntegerToken::new(1),
        end: IntegerToken::new(10),
    };

    let v = MerkleTree::new(r, v, 5.0);
    v.build(&IPartitioner {});

    // let v0 = vec![Token::new(1)];
    // let v1 = vec![Token::new(1), Token::new(1), Token::new(2), Token::new(3)];
    // let v2 = vec![
    //     Token::new(1),
    //     Token::new(1),
    //     Token::new(1),
    //     Token::new(1),
    //     Token::new(1),
    //     Token::new(1),
    // ];
    // let v3 = vec![
    //     Token::new(1),
    //     Token::new(1),
    //     Token::new(1),
    //     Token::new(1),
    //     Token::new(1),
    //     Token::new(1),
    //     Token::new(1),
    //     Token::new(1),
    // ];

    // let s = Token::new(0);
    // let e = Token::new(10000000);
    // let r = Range { start: s, end: e };

    // assert_eq!(MerkleTree::from_vec(r, v0).height(), 1, "item count is 1");
    // assert_eq!(MerkleTree::from_vec(v1).height(), 2, "item count is 4");
    // assert_eq!(MerkleTree::from_vec(v2).height(), 3, "item count is 6");
    // assert_eq!(MerkleTree::from_vec(v3).height(), 3, "item count is 8");
}

// #[test]
// fn test_checking_same_tree() {
//     let v1 = vec![vec![1], vec![1], vec![1], vec![1], vec![1], vec![1]];
//     let v2 = vec![vec![1], vec![1], vec![1], vec![1], vec![1], vec![1]];
//     let v3 = vec![vec![1], vec![1], vec![1], vec![1], vec![1], vec![2]];
//     let v4 = vec![vec![1], vec![1]];

//     let t1 = MerkleTree::from_vec(v1);
//     let t2 = MerkleTree::from_vec(v2);
//     let t3 = MerkleTree::from_vec(v3);
//     let t4 = MerkleTree::from_vec(v4);

//     assert!(t1 == t1);
//     assert!(t1 == t2);
//     assert!(t1 != t3);
//     assert!(t1 != t4);
// }
