use std::ops::Range;
use std::fmt;

use merkle_tree::MerkleTree;
use token::Token;
use node::Node;

use partitioner::Partitioner;

pub fn build<T: Token + PartialOrd + Clone + fmt::Debug>(
    origin: &MerkleTree<T>,
    replica: &MerkleTree<T>,
    p: &Partitioner<Token = T>,
) -> Vec<Range<T>> {
    let ref origin_root = origin.root();
    let ref repli_root = replica.root();

    match (origin_root, repli_root) {
        (&Node::Inner { ref range, .. }, ref r) => {
            if let Some(rr) = r.range_in(range) {
                let mut diff = vec![];
                if origin_root.hash() == rr.hash() {
                    return vec![];
                } else {
                    if Consistency::Non ==
                        difference(origin_root, repli_root, &range, &mut diff, p)
                    {
                        println!("fully inconsistent");
                        diff.push(range.clone());
                        diff
                    } else {
                        diff
                    }
                }
            } else {
                vec![]
            }
        }
        _ => unreachable!(),
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Consistency {
    Complete,
    Partial,
    Non,
}

fn difference<T: Token + PartialOrd + Clone + fmt::Debug>(
    left: &Node<T>,
    right: &Node<T>,
    range: &Range<T>,
    diff: &mut Vec<Range<T>>,
    p: &Partitioner<Token = T>,
) -> Consistency {
    match p.call(&range) {
        Some((ref l, ref r)) => {
            let lc = inner_difference(left, right, l, diff, p);
            let rc = inner_difference(left, right, r, diff, p);

            match (lc, rc) {
                (Consistency::Complete, Consistency::Complete) => Consistency::Complete,
                (Consistency::Non, Consistency::Non) => Consistency::Non,
                (Consistency::Non, _) => {
                    diff.push(l.clone());
                    Consistency::Partial
                }
                (_, Consistency::Non) => {
                    diff.push(r.clone());
                    Consistency::Partial
                }
                (_, _) => Consistency::Partial,
            }
        }
        None => Consistency::Non,
    }
}

fn inner_difference<T: Token + PartialOrd + Clone + fmt::Debug>(
    left: &Node<T>,
    right: &Node<T>,
    range: &Range<T>,
    diff: &mut Vec<Range<T>>,
    p: &Partitioner<Token = T>,
) -> Consistency {
    match (right.range_in(range), left.range_in(range)) {
        (Some(ref rl), Some(ref ll)) => {
            if rl.hash() == ll.hash() {
                println!("{:?} is consistent", range);
                Consistency::Complete
            } else {
                println!("range {:?} is inconsistent", range);
                let d = difference(left, right, range, diff, p);
                println!("difference {:?}", d);
                d
            }
        }
        (None, None) => {
            println!("both can't search {:?}", range);
            Consistency::Complete
        }
        _ => Consistency::Non,
    }
}
