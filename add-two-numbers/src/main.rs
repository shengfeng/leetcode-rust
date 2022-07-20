use add_two_numbers::utils::ListNode;
use std::collections::vec_deque::VecDeque;

pub fn carrier(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, mut carry: i32) -> Option<Box<ListNode>>  {
    if l1.is_none() && l2.is_none() && carry == 0 {
        None
    } else {
        Some(Box::new(ListNode {
            next: carrier(
                l1.and_then(|x| {carry += x.val; x.next}),
                l2.and_then(|x| {carry += x.val; x.next}),
                carry / 10
            ),
            val: carry % 10
        }))
    }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    carrier(l1, l2, 0)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_01() {
        let l1 = ListNode::vec_to_list(VecDeque::from(vec![2, 4, 3]));
        println!("{:?}", l1);
        let l2 = ListNode::vec_to_list(VecDeque::from(vec![5,6,4]));
        println!("{:?}", l2);
        let carry = ListNode::vec_to_list(VecDeque::from(vec![7,0,8]));
        assert_eq!(add_two_numbers(l1, l2), carry);
    }

    #[test]
    fn test_02() {
        let l1 = ListNode::vec_to_list(VecDeque::from(vec![0]));
        let l2 = ListNode::vec_to_list(VecDeque::from(vec![0]));
        let carry = l1.clone();
        assert_eq!(add_two_numbers(l1, l2), carry);
    }
}

fn main() {
    let l1 = ListNode::vec_to_list(VecDeque::from(vec![2,4,3]));
    println!("{:?}", l1);
    let l2 = ListNode::vec_to_list(VecDeque::from(vec![2,4,5]));
    println!("{:?}", l2);
}
