use add_two_numbers_ii::utils::ListNode;

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1_ptr = &l1;
    let mut l2_ptr = &l2;
    let mut l1_nums = vec![];
    let mut l2_nums = vec![];

    while l1_ptr.is_some() || l2_ptr.is_some() {
        if let Some(node) = l1_ptr {
            l1_nums.push(node.val);
            l1_ptr = &node.next;
        }

        if let Some(node) = l2_ptr {
            l2_nums.push(node.val);
            l2_ptr = &node.next;
        }
    }

    let (mut root, mut carry) = (ListNode::new(0), 0);
    while !l1_nums.is_empty() || !l2_nums.is_empty() || carry > 0 {
        let l1_num = l1_nums.pop().unwrap_or(0);
        let l2_num = l2_nums.pop().unwrap_or(0);
        let sum = l1_num + l2_num + carry;
        let mut node = ListNode::new(sum % 10);
        node.next = root.next.take();
        root.next = Some(Box::new(node));
        carry = sum / 10;
    }
    root.next
}


#[cfg(test)]
mod tests {
    use add_two_numbers_ii::utils::ListNode;
    use crate::add_two_numbers;

    #[test]
    fn test_01() {
        let l1 = ListNode::vec_to_list(vec![3,4,2,7]);
        let l2 = ListNode::vec_to_list(vec![4,6,5]);
        let ret = ListNode::vec_to_list(vec![7,0,8,7]);
        assert_eq!(add_two_numbers(l1, l2), ret);
    }

    #[test]
    fn test_02() {
        let l1 = ListNode::vec_to_list(vec![3,4,2]);
        let l2 = ListNode::vec_to_list(vec![4,6,5]);
        let ret = ListNode::vec_to_list(vec![7,0,8]);
        assert_eq!(add_two_numbers(l1, l2), ret);
    }

    #[test]
    fn test_03() {
        let l1 = ListNode::vec_to_list(vec![0]);
        let l2 = ListNode::vec_to_list(vec![0]);
        let ret = ListNode::vec_to_list(vec![0]);
        assert_eq!(add_two_numbers(l1, l2), ret);
    }
}

fn main() {
    println!("Hello, world!");
}
