extern crate merkle;

use std::ops::Range;

use merkle::merkle_tree::MerkleTree;
use merkle::token::IntegerToken;
use merkle::row_hash::RowHash;
use merkle::partitioner::IPartitioner;

fn main() {
    let v = IntegerToken::new(1);
    let v2 = IntegerToken::new(2);
    // let v3 = IntegerToken::new(3);
    let v = vec![
        RowHash::new(v, vec![1, 2, 3]),
        RowHash::new(v2, vec![2, 3, 4]),
        // RowHash::new(v3, vec![4, 5, 6]),
    ];

    let r = Range {
        start: IntegerToken::new(1),
        end: IntegerToken::new(10),
    };

    let mut mer = MerkleTree::new(r, 5.0);
    mer.build(&IPartitioner {}, v);
    println!("-------------------------------");
    println!("{:?}", mer);
}
