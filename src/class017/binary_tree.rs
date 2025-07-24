use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn build() -> TreeNode {
    let mut root = TreeNode::new(1);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.left.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root
}

fn pre_order(root: Option<Rc<RefCell<TreeNode>>>) {
    match root {
        Some(tree_node) => {
            println!("{}", tree_node.borrow().val);
            pre_order(tree_node.borrow().left.clone());
            pre_order(tree_node.borrow().right.clone());
        }
        None => {}
    }
}

fn pre_order_iterative(root: Option<Rc<RefCell<TreeNode>>>) {
    let mut stack = vec![];
    stack.push(root);
    while let Some(Some(node)) = stack.pop() {
        println!("{}", node.borrow().val);
        if let Some(right) = node.borrow().right.clone() {
            stack.push(Some(right));
        }
        if let Some(left) = node.borrow().left.clone() {
            stack.push(Some(left));
        }
    }
}

fn in_order_iterative(root: Option<Rc<RefCell<TreeNode>>>) {
    let mut stack = vec![];
    let mut root = root;
    while !stack.is_empty() || root.is_some() {
        if let Some(node) = root {
            stack.push(node.clone());
            root = node.borrow().left.clone();
        } else {
            root = stack.pop();
            println!("{}", root.as_ref().unwrap().borrow().val);
            let cur = root.as_ref().unwrap().borrow().right.clone();
            root = cur;
        }
    }
}

fn pos_order_two_stacks(root: Option<Rc<RefCell<TreeNode>>>) {
    if root.is_none() {
        return;
    }
    let mut stack = vec![];
    let mut collect = vec![];
    stack.push(root);
    while let Some(Some(node)) = stack.pop() {
        collect.push(node.borrow().val);
        if let Some(left) = node.borrow().left.clone() {
            stack.push(Some(left));
        }
        if let Some(right) = node.borrow().right.clone() {
            stack.push(Some(right));
        }
    }
    for val in collect.iter().rev() {
        println!("{}", val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_node() {
        let mut root = build();
        assert_eq!(
            root,
            TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: None,
                        right: None
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                })))
            }
        );
    }

    #[test]
    fn test_pre_order() {
        let root = build();
        // pre_order(Some(Rc::new(RefCell::new(root))));
        pre_order_iterative(Some(Rc::new(RefCell::new(root))));
    }

    #[test]
    fn test_in_order() {
        let root = build();
        in_order_iterative(Some(Rc::new(RefCell::new(root))));
    }

    #[test]
    fn test_pos_order() {
        let root = build();
        pos_order_two_stacks(Some(Rc::new(RefCell::new(root))));
    }
}
