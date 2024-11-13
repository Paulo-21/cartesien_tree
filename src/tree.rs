use std::{borrow::Borrow, cell::RefCell, collections::VecDeque, process::exit, rc::Rc};

enum Direction { 
    Right, Left
}

#[derive(PartialEq, Clone)]
pub struct Node {
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
        println!();
        println!("key {key}, priority : {priority}");
        if self.is_empty() { self.root = Some(Rc::new(RefCell::new(Node::new(key, priority)))); return; }
        let mut current_node = self.root.clone();
        let mut insert_direction = Direction::Left;//by default
        let mut child_current = None;
        while let Some(n) = current_node.as_ref() {
            let mut new_current = None;
                if key == (**n).borrow().key { return; }
                if key < (**n).borrow().key {
                    if (**n).borrow().left_child.is_none() {
                        let new = Rc::new(RefCell::new(Node::newp(key, priority, n.clone())));
                        (**n).borrow_mut().left_child = Some(new);
                        child_current = (**n).borrow().left_child.clone();
                        println!("INSERT at left");
                        break;
                    }
                    new_current = (**n).borrow().left_child.clone();
                }
                else {
                    if (**n).borrow().right_child.is_none() {
                        let new = Rc::new(RefCell::new(Node::new(key, priority)));
                        (**n).borrow_mut().right_child = Some(new);
                        insert_direction = Direction::Right;
                        println!("INSERT at right");
                        child_current = (**n).borrow().right_child.clone();
                        break;
                    }
                    new_current = (**n).borrow().right_child.clone();
                }
            current_node = new_current;
        }
        current_node = child_current;
        //Rotate
        loop {
            match insert_direction {
                Direction::Left => {
                    println!("ROTATE LEFT");
                    if let Some(n) = current_node.as_ref() {
                        if (**n).borrow().left_child.is_none() && (**n).borrow().right_child.is_none() {
                            println!("NOTHING");
                        }
                        if let Some(parent) = (**n).borrow().parent.as_ref() {
                            if (**n).borrow().priority < (**parent).borrow().priority {
                                n.borrow_mut().parent = parent.borrow_mut().parent.take();
                                parent.borrow_mut().parent = Some(n.clone());//Change parent
                                
                                parent.borrow_mut().left_child = n.borrow_mut().right_child.take();
                                n.borrow_mut().right_child = Some(Rc::clone(parent));
                                if (**n).borrow().parent.is_none() {
                                    self.root = Some(n.clone());
                                    break;
                                }
                                match CartesienTree::does_im_left_child(parent, (**n).borrow().key, (**n).borrow().priority) {
                                    true  => insert_direction = Direction::Left,
                                    false => insert_direction = Direction::Right,
                                }
                            } else { break; }
                        } else { break; }
                    }
                    else { exit(1); }
                },
                Direction::Right => {
                    println!("ROTATE RIGHT");
                    if let Some(n) = current_node.as_ref() {
                        if (**n).borrow().left_child.is_none() && (**n).borrow().right_child.is_none() {
                            println!("NOTHING");
                        }
                        if let Some(parent) = (**n).borrow().parent.as_ref() {
                            if (**n).borrow().priority < (**parent).borrow().priority {
                                n.borrow_mut().parent = parent.borrow_mut().parent.take();
                                parent.borrow_mut().parent = Some(n.clone());//Change parent

                                parent.borrow_mut().right_child = n.borrow_mut().left_child.take();
                                n.borrow_mut().left_child = Some(parent.clone());
                                if (**n).borrow().parent.is_none() {
                                    self.root = Some(n.clone());
                                    break;
                                }
                                match CartesienTree::does_im_left_child(&parent, (**n).borrow().key, (**n).borrow().priority) {
                                    true  => insert_direction = Direction::Left,
                                    false => insert_direction = Direction::Right,
                                }
                            } else { break; }
                        } else { break; }
                    }
                    else {
                        println!("NONE1");
                        exit(1);
                    }
                }
            }            
        }

    }
    pub fn does_im_left_child(parent : &Rc<RefCell<Node>>, child_key : u32, child_priority : u32) -> bool {
            if let Some(lc) = (**parent).borrow().left_child.clone() {
                if (*lc).borrow().priority == child_priority && (*lc).borrow().key == child_key {
                    return true;
                }
            }
        false
    }
    pub fn print_bfs(&self) {
        println!("-------------------BFS----------------");
        let mut file = VecDeque::new();
        file.push_back((self.root.clone(), 0));
        let mut current_level = 0;
        while let Some(next) = file.pop_front() {
            if let Some(r) = next.0 {
                if current_level < next.1 {
                    current_level = next.1;
                    println!();
                    print!("Level : {}", next.1);
                }
                print!("(k:{}, p:{}) | ", (*r).borrow().key, (*r).borrow().priority);
                file.push_back(((*r).borrow().left_child.clone(), current_level+1));
                file.push_back(((*r).borrow().right_child.clone(), current_level+1));
            }
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