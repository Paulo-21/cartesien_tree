use std::{collections::VecDeque, fmt::Display};
use std::cmp::Ordering::*;

pub enum TreeError {
    ElementNotFind,
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
    key : K,
    priority : P,
    left_child : *mut Node<K,P>,
    right_child : *mut Node<K,P>,
    parent : *mut Node<K,P>,
}
impl<K,P> Display for Node<K,P>
where K: Display, P: std::fmt::Display 
{   
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(n :{}:{})", self.key, self.priority )
    }
}

impl<K,P> Node<K,P> {
    pub fn new(key : K, priority : P) -> Self {
        Self{ key, priority, left_child: std::ptr::null_mut(), right_child: std::ptr::null_mut(), parent: std::ptr::null_mut() }
    }
    pub fn newp(key : K, priority : P, parent : *mut Node<K,P>) -> Self {
        Self{ key, priority, left_child: std::ptr::null_mut(), right_child: std::ptr::null_mut(), parent }
    }
}
pub struct CartesienTree<K,P> {
    root : *mut Node<K,P>,
}
impl<P> CartesienTree<u32,P> {
    pub fn insert_str(&mut self, key : &str, priority : P) 
    where P : PartialOrd + Copy {
        let keyn: u32= key.chars().fold(0, |acc , x|26*(acc+x as u32));
        self.insert(keyn, priority);
    }
    pub fn insert_char(&mut self, key : char, priority : P)
    where P : PartialOrd + Copy {
        self.insert(key.to_ascii_lowercase() as u32 - 97, priority);
    }
}
impl<K,P> CartesienTree<K,P> {
    pub fn new() -> Self { Self{root : std::ptr::null_mut() } }
    
    pub fn insert(&mut self, key : K, priority : P)
    where K : PartialOrd + Ord + Copy, P : PartialOrd + Copy {
        if self.is_empty() { 
            let new_node = Box::into_raw(Box::new(Node::new(key, priority)));
            self.root = new_node;
            return; 
        }
        let mut current_node = self.root;
        let mut insert_direction = Direction::Left;//by default
        let mut child_current = std::ptr::null_mut();
        loop {
            if !current_node.is_null() {
                unsafe {
                    let nn = &mut *current_node;
                    match key.cmp(&nn.key) {
                        Less => {
                            if nn.left_child.is_null() {
                                let new_node = Box::into_raw(Box::new(Node::newp(key, priority, current_node)));
                                child_current = new_node;
                                nn.left_child = new_node;
                                break;
                            }
                            current_node = nn.left_child;
                        },
                        Greater => {
                            if nn.right_child.is_null() {
                                let new_node = Box::into_raw(Box::new(Node::newp(key, priority, current_node)));
                                child_current = new_node;
                                nn.right_child = new_node;
                                insert_direction = Direction::Right;
                                break;
                            }
                            current_node = nn.right_child;
                        },
                        Equal => return
                    }
                }
            } else {
                break;
            }
        }
        current_node = child_current;
        //Rotate
        while !current_node.is_null() {
            unsafe {
                let nn = &mut *current_node;
                if !nn.parent.is_null() {
                    let parent = nn.parent;
                    let pp = &mut *parent;
                    if nn.priority < pp.priority {
                        match self.rotate(current_node, parent, insert_direction) {
                            Some(insed) => insert_direction = insed,
                            None => return
                        }
                    } else {break;}
                } else { break; }
            }
        }
    }

    fn rotate(&mut self, n_ptr: *mut Node<K,P>, p_ptr: *mut Node<K,P>, mut insert_direction: Direction) -> Option<Direction>
    where K: PartialEq+ Copy, P : PartialEq + Copy {
        unsafe {
            let nn = &mut *n_ptr;
            let pp = &mut *p_ptr;
            match insert_direction {
                Direction::Left => {
                    nn.parent = pp.parent;
                    pp.parent = n_ptr;

                    pp.left_child = nn.right_child;
                    if !pp.left_child.is_null() { 
                        (*pp.left_child).parent = p_ptr;
                    }
                    nn.right_child = p_ptr;         
                },
                Direction::Right => {
                    nn.parent = pp.parent;
                    pp.parent = n_ptr;
                    pp.right_child = nn.left_child;
                    if !pp.right_child.is_null() {
                        (*pp.right_child).parent = p_ptr;
                    }
                    nn.left_child = p_ptr;            
                }
            }
            if nn.parent.is_null() {
                self.root = n_ptr;
                None
            }
            else {
                let parent = nn.parent;
                let test_key = pp.key;
                let test_prio = pp.priority;
                // Need to check if we are the left or right child of our new parent
                if self.does_im_left_child(parent, test_key, test_prio) {
                    insert_direction = Direction::Left;
                    (*parent).left_child = n_ptr;
                } else {
                    insert_direction = Direction::Right;
                    (*parent).right_child = n_ptr;
                }
                Some(insert_direction)
            }
        }
    }
    pub fn does_im_left_child(&self, parent : *mut Node<K,P>, child_key : K, child_priority : P) -> bool 
    where K : PartialEq, P : PartialEq {
        unsafe {
            if !(*parent).left_child.is_null() {
                let lc = (*parent).left_child;
                if (*lc).priority == child_priority && (*lc).key == child_key {
                    return true;
                }
            }
            false
        }
    }
    
    pub fn remove(&mut self, key:K) -> Result<(), TreeError> 
    where K : PartialEq + Copy + Ord, P :  Copy + PartialOrd {
        let to_remove = self.bin_search(key)?;
        loop {
            unsafe {
                if (*to_remove).left_child.is_null() && (*to_remove).right_child.is_null() {
                    if (*to_remove).parent.is_null() {
                        self.root = std::ptr::null_mut();
                    } else {
                        let parent = (*to_remove).parent;
                        if self.does_im_left_child(parent, (*to_remove).key, (*to_remove).priority) {
                            (*parent).left_child = std::ptr::null_mut();
                        } else {
                            (*parent).right_child = std::ptr::null_mut();
                        }
                    }
                    // Deallocate the node
                    Box::from_raw(to_remove);
                    return Ok(());
                }
                else {
                    if (*to_remove).left_child.is_null() {
                        let c = (*to_remove).right_child;
                        let insert_direction = Direction::Right;
                        self.rotate(c, to_remove, insert_direction);
                    }
                    else if (*to_remove).right_child.is_null() {
                        let c = (*to_remove).left_child;
                        let insert_direction = Direction::Left;
                        self.rotate(c, to_remove, insert_direction);
                    }
                    else {
                        let pl = (*(*to_remove).left_child).priority;
                        let pr = (*(*to_remove).right_child).priority;
                        if pl <= pr {
                            let c = (*to_remove).left_child;
                            let insert_direction = Direction::Left;
                            self.rotate(c, to_remove, insert_direction);
                        } else {
                            let c = (*to_remove).right_child;
                            let insert_direction = Direction::Right;
                            self.rotate(c, to_remove, insert_direction);
                        }
                    }
                }
            }
        }
    }
    pub fn print_bfs(&self)
    where K : Display, P : Display {
        let mut file = VecDeque::new();
        file.push_back((self.root, 0, Direction::Left));
        let mut current_level = -1;
        while let Some((node_ptr, level, dir)) = file.pop_front() {
            if !node_ptr.is_null() {
                unsafe {
                    let r = &*node_ptr;
                    if current_level < level {
                        current_level = level;
                        println!();
                        print!("Level : {} |", level);
                    }
                    print!("{} {} | ", r, dir);                
                    file.push_back((r.left_child, current_level+1, Direction::Left));
                    file.push_back((r.right_child, current_level+1, Direction::Right));
                }
            }
        }
        println!();
        println!("-------------------END----------------");
    }
    pub fn bfs(&self) -> Vec<K> where K : Clone + Copy {
        let mut seq = Vec::new();
        let mut file = VecDeque::new();
        file.push_back(self.root);
        while let Some(node_ptr) = file.pop_front() {
            if !node_ptr.is_null() {
                unsafe {
                    let r = &*node_ptr;            
                    seq.push(r.key);
                    file.push_back(r.left_child);
                    file.push_back(r.right_child);
                }
            }
        }
        seq
    }

    pub fn is_empty(&self) -> bool { self.root.is_null() }

    pub fn bin_search(&self, key : K) -> Result<*mut Node<K,P>, TreeError>
    where K : Ord
    {
        if self.is_empty() { return Err(TreeError::ElementNotFind); }
        let mut current_node = self.root;
        loop {
            if !current_node.is_null() {
                unsafe {
                    let n = &*current_node;
                    match key.cmp(&n.key) {
                        Less => current_node = n.left_child,
                        Greater => current_node = n.right_child,
                        Equal => return Ok(current_node),
                    }
                }
            } else { return Err(TreeError::ElementNotFind); }
        }
    }
}

impl<K,P> Drop for CartesienTree<K,P> {
    fn drop(&mut self) {
        unsafe fn drop_node<K,P>(node: *mut Node<K,P>) {
            if !node.is_null() {
                let n = &mut *node;
                drop_node(n.left_child);
                drop_node(n.right_child);
                // Deallocate the node
                Box::from_raw(node);
            }
        }
        unsafe {
            drop_node(self.root);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tree1() {
        let noeuds = [('A', 5),('B', 3),('C', 8),('D', 2),('E', 6),('F', 7),('G', 9),('H', 1),('I', 10),('J', 12)];
        let mut tree = CartesienTree::<u32,u32>::new();
        noeuds.iter().for_each(|(k,p) | tree.insert_char(*k, *p));
        let seq = tree.bfs();
        assert_eq!(seq, [7, 3, 8, 1, 4, 9, 0, 2, 5, 6]);
    }
    #[test]
    fn tree2() {
        let noeuds = [('H', 1),('G', 9),('A', 5),('B', 3),('D', 2),('F', 7),('C', 8),('J', 12),('I', 10),('E', 6)];
        let mut tree = CartesienTree::<u32,u32>::new();
        noeuds.iter().for_each(|(k,p) | tree.insert_char(*k, *p));
        let seq = tree.bfs();
        assert_eq!(seq, [7, 3, 8, 1, 4, 9, 0, 2, 5, 6]);
    }
    #[test]
    fn tree3() {
        let noeuds = [('E', 6),('H', 1),('B', 3),('D', 2),('C', 8),('F', 7),('G', 9),('J', 12),('A', 5),('I', 10)];
        let mut tree = CartesienTree::<u32,u32>::new();
        noeuds.iter().for_each(|(k,p) | tree.insert_char(*k, *p));
        let seq = tree.bfs();
        assert_eq!(seq, [7, 3, 8, 1, 4, 9, 0, 2, 5, 6]);
    }
}
