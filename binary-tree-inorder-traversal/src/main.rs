use std::rc::Rc;
use std::cell::RefCell;

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

pub fn inorder_recursive(root: &Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
    match root {
        Some(node) => {
            inorder_recursive(&node.borrow().left, ret);
            ret.push(node.borrow().val);
            inorder_recursive(&node.borrow().right, ret)
        },
        None => {}
    }
}

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ret = vec![];
    inorder_recursive(&root, &mut ret);
    ret
}

pub fn to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::VecDeque;
    let head = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
    let mut queue = VecDeque::new();
    queue.push_back(head.as_ref().unwrap().clone());


    for children in vec[1..].chunks(2) {
        let parent = queue.pop_front().unwrap();
        if let Some(v) = children[0] {
            parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
            queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
        }

        if children.len() > 1 {
            if let Some(v) = children[1] {
                parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
            }
        }
    }
    head
}


#[macro_export]
macro_rules! tree {
    () => {
        None
    };
    ($($e:expr),*) => {
        {
            let vec = vec![$(stringify!($e)), *];
            let vec = vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
            to_tree(vec)
        }
    };
    ($($e:expr,)*) => {(tree![$($e),*])};
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fn_test01() {
        assert_eq!(
            inorder_traversal(tree![1, None, 2, 3]),
            vec![1, 3, 2]
        );
    }

    #[test]
    fn test_02() {
        assert_eq!(inorder_traversal(tree![]), vec![]);
    }


    #[test]
    fn test_03() {
        assert_eq!(inorder_traversal(tree![1]), vec![1]);
    }
}

fn main() {
    println!("Hello, world!");
}
