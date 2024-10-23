
struct Node {
    key : u32,
    priority : u32,
    left_child : Option<Box<Node>>,
    right_child : Option<Box<Node>>,
}
impl Node {
    fn push(&mut self) {

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
                node.push();
            },
            None => {
                self.root = Some(Node {});
            }
        }
    }
}