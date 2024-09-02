use core::panic;

#[derive(Debug)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    value: i32
}

impl Node  {
    fn new(value: i32) -> Option<Box<Node>> {
        Some(Box::new(Node { left: None, right: None, value}))
    }

    fn left(&mut self) -> &mut Node {
        match self.left.as_mut() {
            Some(e) => e.as_mut(),
            None => panic!("Couldnt find left element")
        }
    }

    fn right(&mut self) -> &mut Node {
        match self.right.as_mut() {
            Some(e) => e.as_mut(),
            None => panic!("Couldnt find right element")
        }
    }

    fn set_left(&mut self, new_node: Option<Box<Node>>) {
        self.left = new_node
    }


    fn set_right(&mut self, new_node: Option<Box<Node>>) {
        self.right = new_node

    }

    fn print(&self) {
        match &self.left {
            Some(branch) => branch.print(),
            None => {}
        }
        match &self.right {
            Some(branch) => branch.print(),
            None => {}
        }
        println!("{}", self.value)
    }
}

fn main() {
    let mut root = Node {
        right: None,
        left: None,
        value: 69
    };
    root.set_left(Node::new(32));
    root.set_right(Node::new(127));

    root.left().set_left(Node::new(576));
    root.right().set_left(Node::new(2137));
    root.right().left().set_right(Node::new(420));
    root.print()
}
