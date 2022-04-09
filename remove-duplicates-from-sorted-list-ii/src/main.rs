#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { 
            val: val, 
            next: None 
        }
    }

    pub fn vec_to_list(value: &mut Vec<i32>) -> Option<Box<ListNode>> {
        match value.pop() {
            Some(x) => Some(Box::new(ListNode {
                val: x,
                next: ListNode::vec_to_list(value)
            })),
            None => None,
        }
    }

    pub fn list_to_vec(mut value: &Option<Box<ListNode>>) -> Vec<i32> {

        let mut v = vec![];
        while let Some(n) = value {
            v.push(n.val);
            value = &n.next;
        }
        v
    }
}

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut res = Some(Box::new(ListNode::new(0)));
    let mut p = res.as_mut().unwrap();
    let mut pre = 101;
    let mut head = head;
    while let Some(mut node) = head {
        head = node.next.take();
        println!("head1 = {:?}", head);
        println!("node = {:?}", node);
        if  (head.is_some() && head.as_ref().unwrap().val == node.val) || node.val == pre {
            pre = node.val;
        } else {
            pre = node.val;
            p.next = Some(node);
            p = p.next.as_mut().unwrap();
        }
    }
    res.as_mut().unwrap().next.take()
}

#[test]
fn test_01() {
    let mut nums = vec![1,2,3,3,4,4,5];
    let head = ListNode::vec_to_list(&mut nums);
    let mut ret = vec![1, 2, 5];
    let ans = ListNode::vec_to_list(&mut ret);
    assert_eq!(delete_duplicates(head), ans);
}

#[test]
fn test_02() {
    let mut nums = vec![1, 1, 1, 2, 3];
    let head = ListNode::vec_to_list(&mut nums);
    let mut ret = vec![2, 3];
    let ans = ListNode::vec_to_list(&mut ret);
    assert_eq!(delete_duplicates(head), ans);
}


fn main() {
    let mut nums = vec![1,2,3,3,4,4,5];
    let head = ListNode::vec_to_list(&mut nums);
    println!("head = {:?}", head);

    let ret = delete_duplicates(head);
    println!("ret = {:?}", ret);
}
