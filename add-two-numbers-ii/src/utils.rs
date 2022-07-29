#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    pub fn vec_to_list(mut value: Vec<i32>) -> Option<Box<ListNode>> {
        match value.pop() {
            Some(x) => Some(Box::new(ListNode {
                val: x,
                next: Self::vec_to_list(value),
            })),
            None => None,
        }
    }

    pub fn list_to_vec(mut value: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut ret = vec![];
        while let Some(n) = value {
            ret.push(n.val);
            value = &n.next;
        }
        ret
    }
}