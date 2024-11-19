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
    #[derive(Debug)]
    pub enum Interaction {
        Insertion, Suppression, Search
    }
    use fastrand::Rng;
    use std::time::Instant;
    pub fn insert_tonnes(tree : &mut CartesienTree<u32,u32>, n : u32, interac : &Interaction) {
        let mut rng = Rng::new();
        
        for _ in 0..n {
            let k: u32 = rng.u32(..);
            let p: u32 = rng.u32(..);
            match *interac {
                Interaction::Insertion => tree.insert(k, p),
                Interaction::Search => _ = tree.bin_search(k),
                Interaction::Suppression => tree.insert(k, p),
            }
            
        }
    }
    let mut tree = CartesienTree::new();
    let methode = [Interaction::Insertion, Interaction::Search, Interaction::Suppression];
    for interaction in methode.iter() {
        println!("{:?}", interaction);
        let start = Instant::now();
        insert_tonnes(&mut tree, 1000, interaction);
        println!("1000 nodes in {} ms", start.elapsed().as_millis());
        let start = Instant::now();
        insert_tonnes(&mut tree, 100_000, interaction);
        println!("100 000 nodes in {} ms", start.elapsed().as_millis());
        let start = Instant::now();
        insert_tonnes(&mut tree, 1_000_000, interaction);
        println!("1 000 000 nodes in {} ms", start.elapsed().as_millis());
    }
    
}