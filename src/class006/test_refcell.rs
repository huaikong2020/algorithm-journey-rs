use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Self { value, next: None }
    }
    fn set_next(&mut self, next: Option<Rc<RefCell<Node>>>) {
        self.next = next;
    }
    fn get_next(&self) -> Option<Rc<RefCell<Node>>> {
        self.next.as_ref().map(|v| v.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refcell() {
        let mut node1 = Node::new(1);
        let mut node2 = Node::new(2);
        let mut node3 = Node::new(3);
        let node4 = Node::new(4);

        node3.set_next(Some(Rc::new(RefCell::new(node4))));
        node1.set_next(Some(Rc::new(RefCell::new(node3))));
        node2.set_next(node1.get_next());
        println!("node1: {:?}, node2: {:?}", node1, node2);

        let node5 = Node::new(5);
        let nn = node1.get_next().unwrap();
        nn.borrow_mut().set_next(Some(Rc::new(RefCell::new(node5))));
        println!("node1: {:?}, node2: {:?}", node1, node2);
    }
}
