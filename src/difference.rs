use node::{NodeRange, Node};
use digestible::Digestible;

pub fn build(origin: &Node, replica: &Node) -> Vec<NodeRange> {
    let mut diff = vec![];

    match origin {
        &Node::Inner { ref range, .. } => {
            match (origin.search_in(range), replica.search_in(range)) {
                (Some(ll), Some(rr)) => {
                    if !ll.is_same_digest(rr) {
                        let ret = difference(ll, rr, range, &mut diff);
                        if ret == Consistency::None {
                            diff.push(range.clone())
                        }
                    }
                }
                _ => unreachable!(),     // XXX
            }
        }
        _ => unreachable!(),     // XXX
    }

    diff
}

fn difference(
    left: &Node,
    right: &Node,
    range: &NodeRange,
    diff: &mut Vec<NodeRange>,
) -> Consistency {
    match range.partition_by_mid() {
        None => Consistency::None,
        Some((l, r)) => {
            let lc = inner_difference(left, right, &l, diff);
            let rc = inner_difference(left, right, &r, diff);

            match (lc, rc) {
                (Consistency::Complete, Consistency::Complete) => Consistency::Complete,
                (Consistency::None, Consistency::None) => Consistency::None,
                (Consistency::None, _) => {
                    diff.push(l.clone());
                    Consistency::Partial
                }
                (_, Consistency::None) => {
                    diff.push(r.clone());
                    Consistency::Partial
                }
                (_, _) => Consistency::Partial,
            }
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Consistency {
    Complete,
    Partial,
    None,
}


fn inner_difference(
    left: &Node,
    right: &Node,
    range: &NodeRange,
    diff: &mut Vec<NodeRange>,
) -> Consistency {
    match (right.search_in(range), left.search_in(range)) {
        (Some(rl), Some(ll)) => {
            if rl.is_same_digest(ll) {
                println!("{:?} is consistent", range);
                Consistency::Complete
            } else {
                println!("range {:?} is inconsistent", range);
                let d = difference(left, right, range, diff);
                println!("difference {:?}", d);
                d
            }
        }
        (None, None) => {
            println!("both can't search {:?}", range);
            Consistency::Complete
        }
        _ => Consistency::None,
    }
}
