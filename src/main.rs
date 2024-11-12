
mod tree;
use tree::*;
fn main() {
    let mut tree = CartesienTree::new();
    tree.insert(3, 5);
    tree.insert(32, 9);
    tree.insert(33, 6);
    tree.insert(40, 8);
    tree.insert(50, 1);
    tree.insert(20, 1);
    tree.insert(93, 1);
}