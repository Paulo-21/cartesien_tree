
mod tree;
use tree::*;
fn main() {
    let mut tree = CartesienTree::new();
    tree.insert(3, 5);
    tree.print_bfs();
    tree.insert(32, 9);
    tree.print_bfs();
    tree.insert(33, 6);
    tree.print_bfs();
    tree.insert(40, 8);
    tree.print_bfs();
    tree.insert(50, 1);
    tree.print_bfs();
    tree.insert(20, 1);
    tree.print_bfs();
    tree.insert(93, 1);
    tree.print_bfs();
}