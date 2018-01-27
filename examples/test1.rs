extern crate merkle;

use merkle::merkle_tree::MerkleTree;
use merkle::node::NodeRange;
use merkle::token::Token;

fn main() {
    let t1 = Token::new(1);
    let t2 = Token::new(2);
    let t3 = Token::new(3);
    let t4 = Token::new(4);
    let t5 = Token::new(5);
    let t6 = Token::new(6);
    let t7 = Token::new(7);
    let t8 = Token::new(8);
    let t9 = Token::new(9);

    let v1 = vec![
        (t1.clone(), vec![0, 5, 1]),
        (t2.clone(), vec![1, 5, 1]),
        (t3.clone(), vec![2, 5, 1]),
        (t4.clone(), vec![3, 5, 1]),
        (t5.clone(), vec![4, 5, 1]),
        (t6.clone(), vec![5, 5, 1]),
        (t7.clone(), vec![6, 5, 1]),
        (t8.clone(), vec![7, 5, 1]),
        (t9.clone(), vec![8, 5, 1]),
    ];

    let v2 = vec![
        (t1.clone(), vec![0, 5, 1]),
        (t2.clone(), vec![1, 5, 2]),
        (t3.clone(), vec![2, 5, 2]),
        (t4.clone(), vec![3, 5, 1]),
        (t5.clone(), vec![4, 5, 1]),
        (t6.clone(), vec![5, 5, 2]),
        (t7.clone(), vec![6, 5, 10]),
        (t8.clone(), vec![7, 5, 11]),
        (t9.clone(), vec![8, 5, 1]),
    ];

    let range = NodeRange::new(1, 10);
    let mut m1 = MerkleTree::new(range.clone());
    let mut m2 = MerkleTree::new(range.clone());
    m1.build(&v1);
    m2.build(&v2);
    println!("{:?}", m1.difference(&m2));

}
