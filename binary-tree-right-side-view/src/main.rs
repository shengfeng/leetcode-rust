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

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, depth: usize, ans: &mut Vec<i32>) {
        if node.is_none() {
            return;
        }

        if ans.len() == depth {
            ans.push(node.as_ref().unwrap().borrow().val);
        }

        dfs(&node.as_ref().unwrap().borrow().right, depth + 1, ans);
        dfs(&node.as_ref().unwrap().borrow().left, depth + 1, ans);
    }

    let mut ans = vec![];
    dfs(&root, 0, &mut ans);
    ans
}


#[cfg(test)]
mod tests {
    use crate::right_side_view;
    use super::*;

    #[test]
    fn test_01() {
        let root = tree![1, 2, 3, None, 5, None, 4];
        let ret = vec![1, 3, 4];
        assert_eq!(right_side_view(root), ret);
    }
}


fn main() {
    println!("Hello, world!");
}
