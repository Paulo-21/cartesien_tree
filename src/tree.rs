use std::{cell::RefCell, collections::VecDeque, fmt::Display, rc::Rc};
use std::cmp::Ordering::*;

pub enum SearchError {
    ElementNotFind,
}
enum Direction { 
    Right, Left
}
impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "LEFT"),
            Direction::Right => write!(f, "RIGHT"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Node {
    key : u32,
    priority : u32,
    left_child : Option<Rc<RefCell<Node>>>,
    right_child : Option<Rc<RefCell<Node>>>,
    parent : Option<Rc<RefCell<Node>>>,
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        /*let left = Node::fmt_kp(&self.left_child);
        let right = Node::fmt_kp(&self.right_child);
        let parent = Node::fmt_kp(&self.parent);
        write!(f, "(n :{}:{}, ln {} , rn {}, p {})", self.key, self.priority, left, right, parent )*/
        write!(f, "(n :{}:{})", self.key, self.priority )
    }
}

impl Node {
    fn fmt_kp(node : &Option<Rc<RefCell<Node>>>) -> String {
        if let Some(l) = node.as_ref() {
            let mut k = (*l).borrow().key.to_string();
            k.push_str(":");
            k.push_str((*l).borrow().priority.to_string().as_str());
            k
        }else { "None".to_string() }
    }
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
    pub fn insert_str(&mut self, key : &str, priority : u32) {
        let keyn= key.chars().fold(0, |acc , x|26*(acc+x as u32));
        self.insert(keyn, priority);
    }
    pub fn insert_char(&mut self, key : char, priority : u32) {
        self.insert(key.to_ascii_lowercase() as u32 - 97, priority);
    }
    #[inline]
    pub fn insert(&mut self, key : u32, priority : u32) {
        //println!();
        if self.is_empty() { self.root = Some(Rc::new(RefCell::new(Node::new(key, priority)))); return; }
        let mut current_node = self.root.clone();
        let mut insert_direction = Direction::Left;//by default
        let mut child_current = None;
        //println!("TRY insert : k : {key}, prio : {priority}");
        loop {
            let mut new_current = None;
            if let Some(n) = current_node.as_ref() {
                let mut nn = n.borrow_mut();
                match key.cmp(&nn.key) {
                    Less => {
                        //println!("GO LEFT");
                        if nn.left_child.is_none() {
                            let new = Rc::new(RefCell::new(Node::newp(key, priority, n.clone())));
                            child_current = Some(new.clone());
                            nn.left_child = Some(new);
                            //println!("INSERT at left");
                            //println!("{} {}", nn.key,nn.priority );
                            break;
                        }
                        new_current = nn.left_child.clone();
                    },
                    Greater => {
                        if nn.right_child.is_none() {
                            let new = Rc::new(RefCell::new(Node::newp(key, priority, n.clone())));
                            child_current = Some(new.clone());
                            nn.right_child = Some(new);
                            insert_direction = Direction::Right;
                            //println!("INSERT at right");
                            break;
                        }
                        new_current = nn.right_child.clone();
                    },
                    Equal => return
                }
            }
            current_node = new_current;
        }
        current_node = child_current;
        //Rotate
        loop {
            //println!("-----------INSIDE LOOP------------");
            //self.print_bfs();
            if let Some(n) = current_node.as_ref() {
                let mut nn = n.borrow_mut();
                if let Some(parent) = nn.parent.clone().as_ref() {
                    let mut pp = parent.borrow_mut();
                    if nn.priority < pp.priority {
                        let mut child = None;
                        match insert_direction {
                            Direction::Left => {
                                //println!("ROTATE LEFT");
                                nn.parent = pp.parent.take();
                                pp.parent = Some(n.clone());

                                pp.left_child = nn.right_child.take();
                                child = pp.left_child.clone();
                                nn.right_child = Some(Rc::clone(parent));
                                
                            },
                            Direction::Right => {
                                //println!("ROTATE RIGHT");
                                nn.parent = pp.parent.take();
                                pp.parent = Some(n.clone());//Change parent

                                pp.right_child = nn.left_child.take();
                                child = pp.right_child.clone();
                                nn.left_child = Some(Rc::clone(parent));
                                
                            }
                        }
                        if let Some(c) = child.as_ref() {
                            c.borrow_mut().parent = Some(parent.clone());
                        }
                        drop(child);
                        if nn.parent.is_none() {
                            self.root = Some(n.clone());
                            return;
                        }
                        let r = nn.parent.clone();
                        let test_key = pp.key;
                        let test_prio = pp.priority;
                        //println!("N : {} {}", nn.key, nn.priority);
                        //println!("P : {} {}", test_key, test_prio);

                        drop(pp);
                        drop(nn);
                        if let Some(c) = r.as_ref() {
                            //println!("PPP : {} {}",c.borrow().key, c.borrow().priority);
                            match CartesienTree::does_im_left_child(c, test_key, test_prio) {
                                true  => {
                                    insert_direction = Direction::Left;
                                    c.borrow_mut().left_child = Some(Rc::clone(n));
                                },
                                false => {
                                    insert_direction = Direction::Right;
                                    c.borrow_mut().right_child = Some(Rc::clone(n));
                                },
                            }
                        }
                    } else {break;}
                } else { break; }
            } else { break; }                  
        }
    }
    pub fn does_im_left_child(parent : &Rc<RefCell<Node>>, child_key : u32, child_priority : u32) -> bool {
            if let Some(lc) = (**parent).borrow().left_child.clone() {
                let lcc = lc.borrow();
                if lcc.priority == child_priority && lcc.key == child_key {
                    return true;
                }
            }
        false
    }
    
    pub fn remove() -> Result<(), SearchError>{

        Ok(())
    }
    pub fn print_bfs(&self) {
        //println!("-------------------BFS----------------");
        let mut file = VecDeque::new();
        file.push_back((self.root.clone(), 0, Direction::Left));
        let mut current_level = -1;
        while let Some(next) = file.pop_front() {
            if let Some(r) = next.0 {
                if current_level < next.1 {
                    current_level = next.1;
                    println!();
                    print!("Level : {} |", next.1);
                }
                print!("{} {} | ", r.borrow(), next.2);                
                file.push_back(((*r).borrow().left_child.clone(), current_level+1, Direction::Left));
                file.push_back(((*r).borrow().right_child.clone(), current_level+1, Direction::Right));
            }
        }
        println!();
        println!("-------------------END----------------");
    }
    pub fn bfs(&self) -> Vec<u32> {
        let mut seq = Vec::new();
        let mut file = VecDeque::new();
        file.push_back(self.root.clone());
        while let Some(next) = file.pop_front() {
            if let Some(r) = next.as_ref() {              
                seq.push(r.borrow().key);
                file.push_back(r.borrow().left_child.clone());
                file.push_back(r.borrow().right_child.clone());
            }
        }
        seq
    }

    pub fn is_empty(&self) -> bool { self.root.is_none() }

    pub fn search(&self, key : u32) -> Option<(u32,u32)>{
        if self.is_empty() { return None; }
        let mut current_node = self.root.clone();
        loop {
            match current_node {
                Some(n) => {
                    match key.cmp(&n.borrow().key) {
                        Less => current_node = n.borrow().left_child.clone(),
                        Greater => current_node = n.borrow().right_child.clone(),
                        Equal => return Some((n.borrow().key, n.borrow().priority)),
                    }
                },
                None => { return None; }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tree1() {
        let noeuds = [('A', 5),('B', 3),('C', 8),('D', 2),('E', 6),('F', 7),('G', 9),('H', 1),('I', 10),('J', 12)];
        let mut tree = CartesienTree::new();
        noeuds.iter().for_each(|(k,p) | tree.insert_char(*k, *p));
        let seq = tree.bfs();
        assert_eq!(seq, [7, 3, 8, 1, 4, 9, 0, 2, 5, 6]);
    }
    #[test]
    fn tree2() {
        let noeuds = [('H', 1),('G', 9),('A', 5),('B', 3),('D', 2),('F', 7),('C', 8),('J', 12),('I', 10),('E', 6)];
        let mut tree = CartesienTree::new();
        noeuds.iter().for_each(|(k,p) | tree.insert_char(*k, *p));
        let seq = tree.bfs();
        assert_eq!(seq, [7, 3, 8, 1, 4, 9, 0, 2, 5, 6]);
    }
    #[test]
    fn tree3() {
        let noeuds = [('E', 6),('H', 1),('B', 3),('D', 2),('C', 8),('F', 7),('G', 9),('J', 12),('A', 5),('I', 10)];
        let mut tree = CartesienTree::new();
        noeuds.iter().for_each(|(k,p) | tree.insert_char(*k, *p));
        let seq = tree.bfs();
        assert_eq!(seq, [7, 3, 8, 1, 4, 9, 0, 2, 5, 6]);
    }
}