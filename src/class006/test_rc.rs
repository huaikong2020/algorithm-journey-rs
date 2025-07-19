use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Self { value, next: None }
    }
    fn set_next(&mut self, next: Option<Rc<Node>>) {
        self.next = next;
    }
    fn get_next(&self) -> Option<Rc<Node>> {
        self.next.as_ref().map(|v| v.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc() {
        let mut n1 = Node::new(1);
        let mut n2 = Node::new(2);
        let mut n3 = Node::new(3);
        let n4 = Node::new(4);
        n3.set_next(Some(Rc::new(n4)));
        n1.set_next(Some(Rc::new(n3)));
        n2.set_next(n1.get_next());
        println!("n1: {:?}, n2: {:?}", n1, n2);

        let n5 = Node::new(5);
        let nn = n1.get_next().unwrap();
        // nn.set_next(Some(Rc::new(n5))); //nn cannot borrow as mutable
    }
}
