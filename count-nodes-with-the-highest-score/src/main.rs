pub fn count_highest_score_nodes(parents: Vec<i32>) -> i32 {
    let n = parents.len();
    let mut children: Vec<Vec<i32>> = vec![vec![]; n];
    for i in 0..n {
        let p = parents[i];
        if p != -1 {
            children[p as usize].push(i as i32
            );
        }
    }

    fn dfs(node: usize, 
            children: &Vec<Vec<i32>>,
            max_score: &mut usize,
            cnt: &mut i32) -> usize {
        let mut score = 1;
        let mut size = children.len() - 1;
        for c in children[node].iter() {
            let t = dfs(*c as usize, children, max_score, cnt);
            score = score * t;
            size = size - t;
        }
        if node != 0 {
            score = score * size;
        }
        if score == *max_score {
            *cnt += 1;
        } else if score > *max_score {
            *max_score = score;
            *cnt = 1;
        }

        children.len() - size
    }
    
    let mut max_score = 0;
    let mut cnt = 0;
    dfs(0, &children, &mut max_score, &mut cnt);
    cnt
}


use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];
    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            ')' | ']' | '}' => 
                if Some(c) != stack.pop() {
                    return false
                },
            _ => ()
        }
    }
    stack.len() == 0
}

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

pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    if root == None {
        return String::new();
    }

    let node = root.unwrap();
    let ret = node.as_ref().borrow().val.to_string();
    let mut node_borrow = node.as_ref().borrow_mut();
    let left = tree2str(node_borrow.left.take());
    let right = tree2str(node_borrow.right.take());

    match (left.len(), right.len()) {
        (0, 0) => ret,
        (0, _) => format!("{}(){}", ret, right),
        (_, 0) => format!("{}({})", ret, left),
        (_, _) => format!("{}({})({})", ret, left, right)
    }
}

fn main() {
    let parents = vec![-1, 2, 0, 2, 0];
    let result = count_highest_score_nodes(parents);
    println!("{}", result);
}

