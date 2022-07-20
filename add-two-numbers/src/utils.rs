use std::collections::vec_deque::VecDeque;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    pub fn vec_to_list(mut value: VecDeque<i32>) -> Option<Box<ListNode>> {
        match value.pop_front() {
            Some(x) => Some(Box::new(ListNode {
                val: x,
                next: ListNode::vec_to_list(value),
            })),
            None => None,
        }
    }

    pub fn list_to_vec(value: Option<Box<ListNode>>, mut v_cup: VecDeque<i32>) -> VecDeque<i32> {
        match value {
            Some(x) => {
                v_cup.push_back(x.val);
                Self::list_to_vec(x.next, v_cup)
            }
            None => v_cup,
        }
    }
}