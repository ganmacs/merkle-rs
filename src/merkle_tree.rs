use node::{Node, NodeRange};
use token::Token;
use difference;

#[derive(Clone, Debug)]
pub struct MerkleTree {
    root: Option<Node>,
    tree_range: NodeRange,
}

impl MerkleTree {
    pub fn new(range: NodeRange) -> Self {
        MerkleTree {
            root: None,
            tree_range: range,
        }
    }

    pub fn build<T>(&mut self, rows: &Vec<(Token<T>, Vec<u8>)>) {
        if self.root.is_none() {
            self.root = Some(self.build_node(&self.tree_range));
        }
        self.insert_nodes(rows);
    }

    fn build_node(&self, range: &NodeRange) -> Node {
        match range.partition_by_mid() {
            None => Node::empty_leaf(),
            Some((l, r)) => {
                let ll = self.build_node(&l);
                let rr = self.build_node(&r);
                Node::new_node(range.clone(), ll, rr)
            }
        }
    }

    fn insert_nodes<T>(&mut self, rows: &Vec<(Token<T>, Vec<u8>)>) {
        match self.root.as_mut() {
            Some(root) => {
                for r in rows {
                    root.update(&r.0, &r.1)
                }
            }
            None => println!("root of tree should be set"),
        }
    }

    pub fn difference(&self, other: &Self) -> Vec<NodeRange> {
        match (self.root.as_ref(), other.root.as_ref()) {
            (Some(root), Some(other)) => difference::build(root, other),
            _ => panic!("Both merkle tree should be set"),
        }
    }
}
