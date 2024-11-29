mod tree;
use std::{cell::RefCell, rc::Rc};

use tree::{CartesienTree, Node};
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

fn manually_construte_1a() {
    let h = Rc::new(RefCell::new(Node{key : 7, priority : 1, left_child : None, right_child:None, parent : None}));
    let d = Rc::new(RefCell::new(Node{key : 3, priority : 2, left_child : None, right_child:None, parent : None}));
    let b = Rc::new(RefCell::new(Node{key : 1, priority : 3, left_child : None, right_child:None, parent : None}));
    let a = Rc::new(RefCell::new(Node{key : 0, priority : 5, left_child : None, right_child:None, parent : None}));
    let c = Rc::new(RefCell::new(Node{key : 2, priority : 8, left_child : None, right_child:None, parent : None}));
    let e = Rc::new(RefCell::new(Node{key : 4, priority : 6, left_child : None, right_child:None, parent : None}));
    let f = Rc::new(RefCell::new(Node{key : 5, priority : 7, left_child : None, right_child:None, parent : None}));
    let g = Rc::new(RefCell::new(Node{key : 6, priority : 9, left_child : None, right_child:None, parent : None}));
    let i = Rc::new(RefCell::new(Node{key : 8, priority : 10, left_child : None, right_child:None, parent : None}));
    let j = Rc::new(RefCell::new(Node{key : 9, priority : 12, left_child : None, right_child:None, parent : None}));
    h.borrow_mut().left_child = Some(d.clone());
    h.borrow_mut().right_child = Some(i.clone());
    d.borrow_mut().left_child = Some(b.clone());
    d.borrow_mut().right_child = Some(e.clone());
    i.borrow_mut().right_child = Some(j.clone());
    b.borrow_mut().left_child = Some(a.clone());
    b.borrow_mut().right_child = Some(c.clone());
    e.borrow_mut().right_child = Some(f.clone());
    f.borrow_mut().right_child = Some(g.clone());
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
        let stat = tree.depth_stat();
        println!("Stat depth | mean : {:.3}, variance : {:.3}, cv : {:.3}", stat.0, stat.1, (stat.1.sqrt()/stat.0)*100.);
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
