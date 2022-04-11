use std::{rc::Rc, cell::RefCell};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) ->  Self {
        TreeNode { 
            val, 
            left: None, 
            right: None 
        }
    }
}

pub fn check(
    s: &Option<Rc<RefCell<TreeNode>>>, 
    t: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    if s.is_none() && t.is_none() {
        return true;
    }
    if s.is_none() || t.is_none() {
        return false;
    }
    
    let s_borrow = s.as_ref().unwrap().borrow();
    let s_val = s_borrow.val;
    let s_left = &s_borrow.left;
    let s_right = &s_borrow.right;
    let t_borrow = t.as_ref().unwrap().borrow();
    let t_val = t_borrow.val;
    let t_left = &t_borrow.left;
    let t_right = &t_borrow.right;

    if s_val == t_val {
        return check(s_left, t_left) && check(s_right, t_right);
    }
    return false;
}

pub fn dfs(
    s: &Option<Rc<RefCell<TreeNode>>>, 
    t: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(node) = s {
        return check(s, t) || dfs(&node.borrow().left, t) || dfs(&node.borrow().right, t);
    }
    false
}


pub fn is_subtree(
    root: Option<Rc<RefCell<TreeNode>>>, 
    sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    dfs(&root, &sub_root)
}


#[test]
fn test_01() {
    let sub_tree = Some(Rc::new(RefCell::new(TreeNode { 
            val: 4, 
            left: Some(Rc::new(RefCell::new(TreeNode { val: 1, left: None, right: None }))), 
            right: Some(Rc::new(RefCell::new(TreeNode { val: 2, left: None, right: None }))),  
        })));
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: sub_tree.clone(),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: None,
        }))),
    })));
    assert_eq!(is_subtree(tree, sub_tree), true);
}


fn main() {
    println!("Hello, world!");
}
