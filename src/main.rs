extern crate merkle;

use std::ops::Range;

use merkle::merkle_tree::MerkleTree;
use merkle::token::IntegerToken;
use merkle::row_hash::RowHash;
use merkle::partitioner::IPartitioner;

fn main() {
    let v = IntegerToken::new(1);
    let v2 = IntegerToken::new(2);
    let v3 = IntegerToken::new(3);
    let v4 = IntegerToken::new(4);
    let v5 = IntegerToken::new(5);
    let v6 = IntegerToken::new(6);
    let v7 = IntegerToken::new(7);
    let v8 = IntegerToken::new(8);
    let v9 = IntegerToken::new(9);

    let rh = vec![
        RowHash::new(v.clone(), vec![1, 2, 3]),
        RowHash::new(v2.clone(), vec![2, 3, 4]),
        RowHash::new(v3.clone(), vec![4, 5, 6]),
        RowHash::new(v4.clone(), vec![4, 5, 10]),
        RowHash::new(v5.clone(), vec![4, 5, 11]),
        RowHash::new(v6.clone(), vec![4, 5, 12]),
        RowHash::new(v7.clone(), vec![4, 5, 12]),
        RowHash::new(v8.clone(), vec![4, 5, 12]),
        RowHash::new(v9.clone(), vec![4, 5, 12]),
    ];

    let rh2 = vec![
        RowHash::new(v.clone(), vec![1, 2, 3]),
        RowHash::new(v2.clone(), vec![2, 3, 4]),
        RowHash::new(v3.clone(), vec![4, 5, 6]),
        RowHash::new(v4.clone(), vec![4, 5, 10, 1]),
        RowHash::new(v5.clone(), vec![4, 5, 11]),
        RowHash::new(v6.clone(), vec![4, 5, 12, 1]),
        RowHash::new(v7.clone(), vec![4, 5, 12, 1]),
        RowHash::new(v8.clone(), vec![4, 5, 12]),
        RowHash::new(v9.clone(), vec![4, 5, 12, 1]),
    ];

    let r = Range {
        start: IntegerToken::new(1),
        end: IntegerToken::new(10),
    };

    let mut mer = MerkleTree::new(r.clone(), 5.0);
    let mut mer2 = MerkleTree::new(r.clone(), 5.0);
    mer.build(&IPartitioner {}, rh);
    mer2.build(&IPartitioner {}, rh2);
    println!("{:?}", mer.difference(&mer2, &IPartitioner {}));
}
