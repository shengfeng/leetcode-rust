use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
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
      right: None
    }
  }
}

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(root) => {
            let left = root.borrow_mut().left.take();
            let right = root.borrow_mut().right.take();
            return max_depth(left).max(max_depth(right)) + 1;
        },
        None => 0
    }
}

fn main() {
    println!("Hello, world!");
}
