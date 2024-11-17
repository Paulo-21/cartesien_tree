pub mod tree;
use tree::CartesienTree;
use rand::{thread_rng, Rng};
use rand;
#[inline]
pub fn insert_tonnes(n : u32) {
    let mut rng = thread_rng();
    let mut tree = CartesienTree::new();
    for _ in 0..n {
        let k: u32 = rng.gen();
        let p: u32 = rng.gen();
        tree.insert(k, p);
    }
}