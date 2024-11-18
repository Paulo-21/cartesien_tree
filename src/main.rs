mod tree;
use tree::CartesienTree;

#[cfg(not(feature = "benchmark"))]
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
    let _ = tree.remove(50);
    tree.print_bfs();
}

#[cfg(feature = "benchmark")]
fn main() {
    use fastrand::Rng;
    use std::time::Instant;
    pub fn insert_tonnes(n : u32) {
        let mut rng = Rng::new();
        let mut tree = CartesienTree::new();
        for _ in 0..n {
            let k: u32 = rng.u32(..);
            let p: u32 = rng.u32(..);
            tree.insert(k, p);
        }
    }
    let start = Instant::now();
    insert_tonnes(1000);
    println!("1000 nodes in {} ms", start.elapsed().as_millis());
    let start = Instant::now();
    insert_tonnes(100_000);
    println!("100 000 nodes in {} ms", start.elapsed().as_millis());
    let start = Instant::now();
    insert_tonnes(1_000_000);
    println!("1 000 000 nodes in {} ms", start.elapsed().as_millis());
}