use hash_tree::HashTree;
use digest::Digestible;

#[derive(Clone, Debug)]
pub struct MerkleTree<T> {
    root: HashTree<T>,
    count: usize,
    height: usize,
}

impl<T> PartialEq for MerkleTree<T> {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.height == other.height && self.root == other.root
    }
}

impl<T> MerkleTree<T> {
    pub fn from_vec(vec: Vec<T>) -> Self
    where
        T: Digestible,
    {
        let count = vec.len();
        let mut height = 0;
        // XXX
        let mut nodes: Vec<HashTree<T>> = vec.into_iter().map(|v| HashTree::new_leaf(v)).collect();
        while nodes.len() > 1 {
            let mut next = Vec::new();
            while let Some(n1) = nodes.pop() {
                match nodes.pop() {
                    Some(n2) => next.push(HashTree::new_node(n1, n2)),
                    None => next.push(n1),
                };
            }
            nodes = next;
            height += 1;
        }

        if height == 0 {
            height += 1
        }

        let root = nodes.remove(0);
        MerkleTree {
            root: root,
            count: count,
            height: height,
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

#[test]
fn test_height_of_tree() {
    let v0 = vec![vec![1]];
    let v1 = vec![vec![1], vec![1], vec![2], vec![3]];
    let v2 = vec![vec![1], vec![1], vec![1], vec![1], vec![1], vec![1]];
    let v3 = vec![
        vec![1],
        vec![1],
        vec![1],
        vec![1],
        vec![1],
        vec![1],
        vec![1],
        vec![1],
    ];

    assert_eq!(MerkleTree::from_vec(v0).height(), 1, "item count is 1");
    assert_eq!(MerkleTree::from_vec(v1).height(), 2, "item count is 4");
    assert_eq!(MerkleTree::from_vec(v2).height(), 3, "item count is 6");
    assert_eq!(MerkleTree::from_vec(v3).height(), 3, "item count is 8");
}

#[test]
fn test_checking_same_tree() {
    let v1 = vec![vec![1], vec![1], vec![1], vec![1], vec![1], vec![1]];
    let v2 = vec![vec![1], vec![1], vec![1], vec![1], vec![1], vec![1]];
    let v3 = vec![vec![1], vec![1], vec![1], vec![1], vec![1], vec![2]];
    let v4 = vec![vec![1], vec![1]];

    let t1 = MerkleTree::from_vec(v1);
    let t2 = MerkleTree::from_vec(v2);
    let t3 = MerkleTree::from_vec(v3);
    let t4 = MerkleTree::from_vec(v4);

    assert!(t1 == t1);
    assert!(t1 == t2);
    assert!(t1 != t3);
    assert!(t1 != t4);
}
