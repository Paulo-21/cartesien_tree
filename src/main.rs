mod tree;
use tree::CartesienTree;
#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

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
}

#[cfg(feature = "benchmark")]
fn main() {
    #[derive(Debug, PartialEq)]
    pub enum Interaction {
        Insertion, Suppression, Search
    }
    use fastrand::Rng;
    use std::time::Instant;
    fn bench_insert(n : u32) -> CartesienTree<u32,u32> {
        let start = Instant::now();
        let mut tree = CartesienTree::new();
        let mut rng = Rng::new();
        for i in 0..n {
            let k = rng.u32(..);
            let p = rng.u32(..);
            tree.insert(k, p);
        }
        println!("{n} nodes in {} msec", start.elapsed().as_millis());
        tree
    }
    fn bench_search(tree : &mut CartesienTree<u32,u32>, n : u32) {
        let mut rng = Rng::new();
        let start = Instant::now();
        for _ in 0..n {
            let k = rng.u32(..);
            let _ = tree.bin_search(k);
        }
        println!("{n} nodes in {} msec", start.elapsed().as_millis());
    }
    fn bench_remove(tree : &mut CartesienTree<u32,u32>, n : u32) {
        let mut rng = Rng::new();
        let start = Instant::now();
        for _ in 0..n {
            let k = rng.u32(..);
            let _ = tree.remove(k);
        }
        println!("{n} nodes in {} msec", start.elapsed().as_millis());
    }
    let methode = [Interaction::Insertion, Interaction::Search, Interaction::Suppression];
    let nb_noeuds = [1000, 100_000, 1_000_000, 10_000_000];
    let mut abr: Vec<CartesienTree<u32, u32>> = Vec::with_capacity(nb_noeuds.len());
    for interaction in methode.iter() {
        println!("{:?}", interaction);
        for (i, n) in nb_noeuds.iter().enumerate() {
            let a = match *interaction {
                Interaction::Insertion => Some(bench_insert(*n)),
                Interaction::Search => { bench_search(&mut abr[i], *n); None},
                Interaction::Suppression => { bench_remove(&mut abr[i], *n); None},
            };
            if let Some(arbre) = a {
                abr.push(arbre);
            }
        }
    }
    
}
