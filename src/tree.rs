use std::cell::RefMut;
use std::{cell::RefCell, collections::VecDeque, fmt::Display, rc::Rc};
use std::cmp::Ordering::*;

pub enum TreeError {
    ElementNotFind,
    NotExist
}
#[derive(Clone, Copy)]
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
pub struct Node<K,P> {
    pub key : K,
    pub priority : P,
    pub left_child : Option<Rc<RefCell<Node<K,P>>>>,
    pub right_child : Option<Rc<RefCell<Node<K,P>>>>,
    pub parent : Option<Rc<RefCell<Node<K,P>>>>,
}
impl<K,P> Display for Node<K,P>
where K: Display, P: std::fmt::Display 
{   
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(n :{}:{})", self.key, self.priority )
    }
}

impl<K,P> Node<K,P> {
    fn fmt_kp(node : &Option<Rc<RefCell<Node<K,P>>>>) -> String
    where K : ToString, P : ToString {
        if let Some(l) = node.as_ref() {
            let mut k = (*l).borrow().key.to_string();
            k.push(':');
            k.push_str((*l).borrow().priority.to_string().as_str());
            k
        }else { "None".to_string() }
    }
    pub fn new(key : K, priority : P) -> Self {
        Self{ key, priority, left_child: None, right_child: None, parent:None }
    }
    pub fn newp(key : K, priority : P, parent : Rc<RefCell<Node<K,P>>>) -> Self {
        Self{ key, priority, left_child: None, right_child: None, parent:Some(parent) }
    }
}
pub struct CartesienTree<K,P> {
    root : Option<Rc<RefCell<Node<K,P>>>>,
}
impl<P> CartesienTree<u32,P> {
    pub fn insert_char(&mut self, key : char, priority : P)
    where P : PartialOrd + Copy {
        self.insert(key.to_ascii_lowercase() as u32 - 97, priority);
    }
    pub fn insert_str(&mut self, key : &str, priority : P) 
    where P : PartialOrd + Copy {
        let keyn: u32= key.chars().fold(0, |acc , x|26*(acc+x as u32));
        self.insert(keyn, priority);
    }
    pub fn remove_char(&mut self, key : char)
    where P : PartialOrd + Copy {
        _ = self.remove(key.to_ascii_lowercase() as u32 - 97);
    }
    pub fn remove_str(&mut self, key : &str)
    where P : PartialOrd + Copy {
        let keyn: u32= key.chars().fold(0, |acc , x|26*(acc+x as u32));
        _ = self.remove(keyn);
    }
}
impl<K,P> CartesienTree<K,P> {
    pub fn new() -> Self { Self{root : None} }
    pub fn get_left_child(&self, k : K) -> Result<Rc<RefCell<Node<K,P>>>, TreeError> 
    where K : Ord {
        let noeud = self.bin_search(k)?;
        if let Some(c) = noeud.borrow().left_child.as_ref() {
            return Ok(c.clone());
        }
        Err(TreeError::NotExist)
    }
    pub fn get_right_child(&self, k : K) -> Result<Rc<RefCell<Node<K,P>>>, TreeError> 
    where K : Ord {
        let noeud = self.bin_search(k)?;
        if let Some(c) = noeud.borrow().left_child.as_ref() {
            return Ok(c.clone());
        }
        Err(TreeError::NotExist)
    }
    
    pub fn insert(&mut self, key : K, priority : P)
    where K : PartialOrd + Ord + Copy, P : PartialOrd + Copy {
        if self.is_empty() { self.root = Some(Rc::new(RefCell::new(Node::new(key, priority)))); return; }
        let mut current_node = self.root.clone();
        let mut insert_direction = Direction::Left;//by default
        let mut child_current = None;
        loop {
            let mut new_current = None;
            if let Some(n) = current_node.as_ref() {
                let mut nn = n.borrow_mut();
                match key.cmp(&nn.key) {
                    Less => {
                        if nn.left_child.is_none() {
                            let new = Rc::new(RefCell::new(Node::newp(key, priority, n.clone())));
                            child_current = Some(new.clone());
                            nn.left_child = Some(new);
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
        if let Some(n) = current_node.as_ref() {
            loop {
                let nn = n.borrow_mut();
                    if let Some(parent) = nn.parent.clone().as_ref() {
                        let pp = parent.borrow_mut();
                        if nn.priority < pp.priority {
                            match CartesienTree::rotate(self, nn, pp, insert_direction, n.clone(), parent.clone()) {
                                Some(insed) => insert_direction = insed,
                                None => return
                            }
                        } else {break;}
                    } else { break; }      
            }
        }
    }
    #[inline]
    fn rotate(&mut self, mut nn : RefMut<'_, Node<K,P>>, mut pp : RefMut<'_, Node<K,P>>, mut insert_direction :Direction, n : Rc<RefCell<Node<K,P>>>, parent :Rc<RefCell<Node<K,P>>> ) -> Option<Direction> 
    where K: PartialEq+ Copy, P : PartialEq + Copy {
        match insert_direction {
            Direction::Left => {
                nn.parent = pp.parent.take();
                pp.parent = Some(n.clone());

                pp.left_child = nn.right_child.take();
                if let Some(c) = pp.left_child.as_ref() { 
                    c.borrow_mut().parent = Some(parent.clone());
                }
                nn.right_child = Some(parent);         
            },
            Direction::Right => {
                nn.parent = pp.parent.take();
                pp.parent = Some(n.clone());//Change parent
                pp.right_child = nn.left_child.take();
                if let Some(c) = pp.right_child.as_ref() {
                    c.borrow_mut().parent = Some(parent.clone());
                }
                nn.left_child = Some(parent);            
            }
        }
        if nn.parent.is_none() {
            self.root = Some(n);
            None
        }
        else {
            let r = nn.parent.as_ref();
            let test_key = pp.key;
            let test_prio = pp.priority;
            drop(pp);
            //drop(nn);
            if let Some(c) = r {
                match CartesienTree::does_im_left_child(c, test_key, test_prio) {
                    true  => {
                        insert_direction = Direction::Left;
                        c.borrow_mut().left_child = Some(n);
                    },
                    false => {
                        insert_direction = Direction::Right;
                        c.borrow_mut().right_child = Some(n);
                    },
                }
            }
            Some(insert_direction)
        }

    }
    #[inline]
    pub fn does_im_left_child(parent : &Rc<RefCell<Node<K,P>>>, child_key : K, child_priority : P) -> bool 
    where K : PartialEq, P : PartialEq {
            if let Some(lc) = parent.borrow().left_child.as_ref() {
                let lcc = lc.borrow();
                if lcc.priority == child_priority && lcc.key == child_key {
                    return true;
                }
            }
        false
    }
    
    pub fn remove(&mut self, key:K) -> Result<(), TreeError> 
    where K : PartialEq + Copy + Ord, P :  Copy + PartialOrd {
        let to_remove = self.bin_search(key)?;
        loop {
            if to_remove.borrow().left_child.is_some() && to_remove.borrow().right_child.is_some() {
                let to_remove_ref = to_remove.borrow_mut();
                if let Some(leftc) = to_remove_ref.left_child.as_ref() {
                    if let Some(rightc) = to_remove_ref.right_child.as_ref() {
                        let pl = leftc.borrow().priority;
                        let pr = rightc.borrow().priority;
                        if pl <= pr {
                            let pp = to_remove_ref;
                            let c = pp.left_child.clone().unwrap();
                            let nn = c.borrow_mut();
                            let _ = self.rotate(nn, pp, Direction::Left, c.clone(), to_remove.clone());
                        } else {
                            let pp = to_remove_ref;
                            let c = pp.right_child.clone().unwrap();
                            let nn = c.borrow_mut();
                            let _ = self.rotate(nn, pp, Direction::Right, c.clone(), to_remove.clone());
                        }
                    }
                }
            }
            else if to_remove.borrow().left_child.is_none() && to_remove.borrow().right_child.is_none() {
                if (*to_remove).borrow_mut().parent.is_none() {
                    self.root = None;
                } else {
                    let parent = (*to_remove).borrow_mut().parent.take().unwrap();
                    match CartesienTree::does_im_left_child(&parent, (*to_remove).borrow().key, (*to_remove).borrow().priority) {
                        true => (*parent).borrow_mut().left_child = None,
                        false => (*parent).borrow_mut().right_child = None,
                    }
                }
                drop(to_remove);
                return Ok(());
            }
            else if to_remove.borrow().left_child.is_none() {
                if (*to_remove).borrow_mut().parent.is_none() {
                    self.root = to_remove.borrow_mut().right_child.take();
                } else {
                    let mut rm = to_remove.borrow_mut();
                    let parent = rm.parent.take().unwrap();
                    let fils = rm.right_child.take().unwrap();
                    fils.borrow_mut().parent = Some(parent.clone());
                    drop(rm);
                    match CartesienTree::does_im_left_child(&parent, (*to_remove).borrow().key, (*to_remove).borrow().priority) {
                        true => (*parent).borrow_mut().left_child = Some(fils),
                        false => (*parent).borrow_mut().right_child = Some(fils),
                    }
                }
                drop(to_remove);
                return Ok(());  
            }
            else if to_remove.borrow().right_child.is_none() {
                if (*to_remove).borrow_mut().parent.is_none() {
                    self.root = to_remove.borrow_mut().left_child.take();
                } else {
                    let mut rm = to_remove.borrow_mut();
                    let parent = rm.parent.take().unwrap();
                    let fils = rm.left_child.take().unwrap();
                    fils.borrow_mut().parent = Some(parent.clone());
                    drop(rm);
                    match CartesienTree::does_im_left_child(&parent, (*to_remove).borrow().key, (*to_remove).borrow().priority) {
                        true => (*parent).borrow_mut().left_child = Some(fils),
                        false => (*parent).borrow_mut().right_child = Some(fils),
                    }
                }
                drop(to_remove);
                return Ok(());  
            }
        }
    }
    pub fn depth_stat(&self) -> (f32, f32) {
        let mut profondeur = Vec::new();
        let mut file = VecDeque::new();
        file.push_back((self.root.clone(), 0));
        let mut current_level : i32 = -1;
        while let Some(next) = file.pop_front() {
            if let Some(r) = next.0 {
                profondeur.push(current_level);
                if current_level < next.1 {
                    current_level = next.1;
                }
                file.push_back((r.borrow().left_child.clone(), current_level+1));
                file.push_back((r.borrow().right_child.clone(), current_level+1));
            }
        }
        let mean    : f32 = profondeur.iter().sum::<i32>() as f32 / (profondeur.len() as f32);
        let variance = profondeur.iter().fold(0., |acc, x| acc + ((*x as f32) - mean).powi(2)) / (profondeur.len() as f32);
        (mean, variance)
    }
    pub fn print_bfs(&self)
    where K : Display, P : Display {
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
    pub fn bfs(&self) -> Vec<K> where K : Clone + Copy {
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
    pub fn dfs(&self) -> Vec<K> where K : Clone + Copy {
        let mut seq = Vec::new();
        let mut file = VecDeque::new();
        file.push_back(self.root.clone());
        while let Some(next) = file.pop_back() {
            if let Some(r) = next.as_ref() {              
                seq.push(r.borrow().key);
                file.push_back(r.borrow().right_child.clone());
                file.push_back(r.borrow().left_child.clone());
            }
        }
        seq
    }

    pub fn is_empty(&self) -> bool { self.root.is_none() }

    pub fn bin_search(&self, key : K) -> Result<Rc<RefCell<Node<K,P>>>, TreeError>
    where K : Ord
    {
        if self.is_empty() { return Err(TreeError::ElementNotFind); }
        let mut current_node = self.root.clone();
        loop {
            match current_node {
                Some(n) => {
                    match key.cmp(&n.borrow().key) {
                        Less => current_node = n.borrow().left_child.clone(),
                        Greater => current_node = n.borrow().right_child.clone(),
                        Equal => return Ok(n.clone()),
                    }
                },
                None => { return Err(TreeError::ElementNotFind); }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insertion1() {
        let noeuds = [('A', 5),('B', 3),('C', 8),('D', 2),('E', 6),('F', 7),('G', 9),('H', 1),('I', 10),('J', 12)];
        let mut tree = CartesienTree::<u32,u32>::new();
        noeuds.iter().for_each(|(k,p) | tree.insert_char(*k, *p));
        let seq = tree.bfs();
        let seq_dfs = tree.dfs();
        assert_eq!(seq, [7, 3, 8, 1, 4, 9, 0, 2, 5, 6]);
        assert_eq!(seq_dfs, [7, 3, 1, 0, 2, 4, 5, 6, 8, 9]);
    }
    #[test]
    fn insertion2() {
        let noeuds = [('H', 1),('G', 9),('A', 5),('B', 3),('D', 2),('F', 7),('C', 8),('J', 12),('I', 10),('E', 6)];
        let mut tree = CartesienTree::<u32,u32>::new();
        noeuds.iter().for_each(|(k,p) | tree.insert_char(*k, *p));
        let seq = tree.bfs();
        let seq_dfs = tree.dfs();
        assert_eq!(seq, [7, 3, 8, 1, 4, 9, 0, 2, 5, 6]);
        assert_eq!(seq_dfs, [7, 3, 1, 0, 2, 4, 5, 6, 8, 9]);
    }
    #[test]
    fn insertion3() {
        let noeuds = [('E', 6),('H', 1),('B', 3),('D', 2),('C', 8),('F', 7),('G', 9),('J', 12),('A', 5),('I', 10)];
        let mut tree = CartesienTree::<u32,u32>::new();
        noeuds.iter().for_each(|(k,p) | tree.insert_char(*k, *p));
        let seq = tree.bfs();
        let seq_dfs = tree.dfs();
        assert_eq!(seq, [7, 3, 8, 1, 4, 9, 0, 2, 5, 6]);
        assert_eq!(seq_dfs, [7, 3, 1, 0, 2, 4, 5, 6, 8, 9]);
    }
    #[test]
    fn remove1() {
        let to_delete = [('A',5u32)];
        let noeuds = [('A', 5),('B', 3),('C', 8),('D', 2),('E', 6),('F', 7),('G', 9),('H', 1),('I', 10),('J', 12)];
        let mut tree = CartesienTree::<u32,u32>::new();
        noeuds.iter().for_each(|(k,p) | tree.insert_char(*k, *p));
        to_delete.iter().for_each(|(k,_) | tree.remove_char(*k));
        let seq = tree.bfs();
        let seq_dfs = tree.dfs();
        assert_eq!(seq, [7, 3, 8, 1, 4, 9, 2, 5, 6]);
        assert_eq!(seq_dfs, [7, 3, 1, 2, 4, 5, 6, 8, 9]);
    }
    #[test]
    fn remove2() {
        let to_delete = [('A',5u32), ('J',12),];
        let noeuds = [('A', 5),('B', 3),('C', 8),('D', 2),('E', 6),('F', 7),('G', 9),('H', 1),('I', 10),('J', 12)];
        let mut tree = CartesienTree::<u32,u32>::new();
        noeuds.iter().for_each(|(k,p) | tree.insert_char(*k, *p));
        to_delete.iter().for_each(|(k,_) | tree.remove_char(*k));
        let seq = tree.bfs();
        let seq_dfs = tree.dfs();
        assert_eq!(seq, [7, 3, 8, 1, 4, 2, 5, 6]);
        assert_eq!(seq_dfs, [7, 3, 1, 2, 4, 5, 6, 8]);
    }
    #[test]
    fn remove3() {
        let to_delete = [('A',5u32), ('J',12), ('H', 1)];
        let noeuds = [('A', 5),('B', 3),('C', 8),('D', 2),('E', 6),('F', 7),('G', 9),('H', 1),('I', 10),('J', 12)];
        let mut tree = CartesienTree::<u32,u32>::new();
        noeuds.iter().for_each(|(k,p) | tree.insert_char(*k, *p));
        to_delete.iter().for_each(|(k,_) | tree.remove_char(*k));
        let seq = tree.bfs();
        let seq_dfs = tree.dfs();
        assert_eq!(seq, [3, 1, 4, 2, 5, 6, 8]);
        assert_eq!(seq_dfs, [3, 1, 2, 4, 5, 6, 8]);
    }
}