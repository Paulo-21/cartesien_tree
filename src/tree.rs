
struct Node {
    key : u32,
    priority : u32,
    left_child : Option<Box<Node>>,
    right_child : Option<Box<Node>>,
}
impl Node {
    pub fn new(key : u32, priority : u32) -> Self {
        Self{ key, priority, left_child: None, right_child: None }
    }
    fn push(&mut self, key : u32, priority : u32) {
        
    }
}
pub struct CartesienTree {
    root : Option<Box<Node>>,
}
impl CartesienTree {
    pub fn new() -> Self {
        return Self{root : None};
    }
    pub fn push(&mut self, key : u32, priority : u32) {
        match &mut self.root {
            Some(node) => {
                node.push(key, priority);
            },
            None => {
                self.root = Some(Box::new(Node {key, priority, left_child:None, right_child:None}));
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
    pub fn get_left_child_of(&self, key : u32, priority : u32) -> Box<Node> {
        Box::new(Node::new());
    }
    pub fn get_right_child_of(&self, key : u32, priority : u32) {

    }
}