extern crate merkle;
use merkle::merkle_tree::MerkleTree;

fn main() {
    let v = vec![vec![1], vec![2], vec![3]];
    let k = MerkleTree::from_vec(v);
    println!("{:?}", k);
}
