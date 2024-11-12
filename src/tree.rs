use std::{cell::RefCell, rc::Rc};

enum Direction { 
    Right, Left
}

#[derive(PartialEq, Clone)]
struct Node {
    key : u32,
    priority : u32,
    left_child : Option<Rc<RefCell<Node>>>,
    right_child : Option<Rc<RefCell<Node>>>,
    parent : Option<Rc<RefCell<Node>>>,
}
impl Node {
    pub fn new(key : u32, priority : u32) -> Self {
        Self{ key, priority, left_child: None, right_child: None, parent:None }
    }
    pub fn newp(key : u32, priority : u32, parent : Rc<RefCell<Node>>) -> Self {
        Self{ key, priority, left_child: None, right_child: None, parent:Some(parent) }
    }
}
pub struct CartesienTree {
    root : Option<Rc<RefCell<Node>>>,
}
impl CartesienTree {
    pub fn new() -> Self { Self{root : None} }

    pub fn insert(&mut self, key : u32, priority : u32) {
        if self.is_empty() { self.root = Some(Rc::new(RefCell::new(Node::new(key, priority)))); return; }
        let mut current_node = self.root.clone();
        let mut insert_direction = Direction::Left;//by default
        while let Some(n) = current_node.as_ref() {
            let mut new_current = None;
                println!("HELLO");
                if key == (**n).borrow().key { return; }
                if key < (**n).borrow().key {
                    if (**n).borrow().left_child.is_none() {
                        let new = Rc::new(RefCell::new(Node::newp(key, priority, n.clone())));
                        (**n).borrow_mut().left_child = Some(new);
                        break;
                    }
                    else { new_current = (**n).borrow().left_child.clone(); }
                }
                else {
                    if (**n).borrow().right_child.is_none() {
                        let new = Rc::new(RefCell::new(Node::new(key, priority)));
                        (**n).borrow_mut().right_child = Some(new);
                        insert_direction = Direction::Right;
                        println!("INSERT at right");
                        break;
                    }
                    else { new_current = (**n).borrow().right_child.clone(); }
                }
            
            current_node = new_current;
        }
        //Rotate
        loop {// Je suis partie dans le mauvais sens, le current node doit être le noeud fils d'avant et on remonte le parent
            let mut new_current = None;
            match insert_direction {
                Direction::Left => {
                    if let Some(n) = current_node.as_ref() {
                        let mut o = (**n).borrow_mut();
                        let pprio = o.priority;
                        let left_child = o.left_child.clone();
                        if let Some (n_child) = &left_child {
                            if pprio > (**n_child).borrow().priority {
                                println!("LEFT");
                                o.left_child = (**n_child).borrow_mut().right_child.take();
                                n_child.borrow_mut().right_child = Some(n.clone());
                                std::mem::swap(&mut n_child.borrow_mut().parent, &mut o.parent); //SWAP parent
                                new_current = Some(n_child.clone());
                            }
                            else { break; }
                        }
                        else {
                            println!("NONE2");
                        }
                    }
                    else {
                        println!("NONE1");
                    }
                },
                Direction::Right => {
                    if let Some(n) = current_node.as_ref() {
                        let mut o = (**n).borrow_mut();
                        let pprio = o.priority;
                        let right_child = o.right_child.clone();
                        if let Some (n_child) = &right_child {
                            if pprio > (**n_child).borrow().priority {
                                println!("Do ROTATION");
                                o.right_child = (**n_child).borrow_mut().left_child.take();
                                n_child.borrow_mut().left_child = Some(n.clone());
                                std::mem::swap(&mut n_child.borrow_mut().parent, &mut o.parent); //SWAP parent
                                new_current = Some(n_child.clone());
                            }
                            else { break; }
                        }
                        else {
                            println!("NONE2");
                        }
                    }
                    else {
                        println!("NONE1");
                    }
                }
            }
            current_node = new_current;
        }
        
    }
    pub fn is_empty(&self) -> bool { self.root.is_none() }

    pub fn search(&self, key : u32) -> Option<(u32,u32)>{
        if self.is_empty() { return None; }
        let mut current_node = self.root.clone();
        loop {
            match current_node {
                Some(n) => {
                    if key < (*n).borrow().key { current_node = (*n).borrow().left_child.clone(); }
                    else if key == (*n).borrow().key { return Some(((*n).borrow().key, (*n).borrow().priority)); }
                    else { current_node = (*n).borrow().right_child.clone(); }
                },
                None => { return None; }
            }
        }
    }
}