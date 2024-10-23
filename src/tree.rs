enum Direction {
    Right, Left
}

#[derive(PartialEq, Clone)]
struct Node {
    key : u32,
    priority : u32,
    left_child : Option<Box<Node>>,
    right_child : Option<Box<Node>>,
    parent : Option<Box<Node>>,
}
impl Node {
    pub fn new(key : u32, priority : u32) -> Self {
        Self{ key, priority, left_child: None, right_child: None, parent:None }
    }
    fn is_parent_of(&self, node : Box<Node>) -> (bool, Direction) {
        if let Some(n) = &self.right_child {
            if n.key == node.key && n.priority == node.priority {
                return (true, Direction::Right);
            }
        }
        if let Some(n) = &self.left_child {
            if n.key == node.key && n.priority == node.priority {
                return (true, Direction::Left);
            }
        }
        (false, Direction::Right)
    }
}
pub struct CartesienTree {
    root : Option<Box<Node>>,
}
impl CartesienTree {
    pub fn new() -> Self {
        return Self{root : None};
    }
    fn rotate_tree(mut parent : Box<Node> , new: Box<Node>) {
        let new_p = new.priority;
        loop {
            if parent.priority >=  new_p {
                break;
            }
            let (b, dir) = parent.is_parent_of(new.clone());
        }
    }
    pub fn insert(&mut self, key : u32, priority : u32) { // Pas fini
        if self.is_empty() { return; }
        let mut current_node = self.root;
        loop {
            match current_node {
                Some(n) => {
                    if key == n.key { return; }
                    let cur = 
                    if key < n.key {
                        current_node = &mut n.left_child;
                    }
                    else {
                        current_node = &mut n.right_child; 
                    };
                    if *current_node == None {
                        let new = Box::new(Node::new(key, priority));
                        let p = n.clone();
                        CartesienTree::rotate_tree(p, new.clone());
                        *current_node = Some(new);
                        return;
                    }
                },
                None => { 

                }
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
    pub fn search(&self, key : u32) -> Option<(u32,u32)>{
        if self.is_empty() {
            return None;
        }
        let mut current_node = &self.root;
        loop {
            match current_node {
                Some(n) => {
                    if key < n.key { current_node = &n.left_child; }
                    else if key == n.key { return Some((n.key, n.priority)); }
                    else { current_node = &n.right_child; }
                },
                None => { return None; }
            }
        }
    }
}