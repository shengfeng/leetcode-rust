use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

struct ListNode {
    key: i32,
    val: i32,
    prev: Option<Rc<RefCell<ListNode>>>,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(key: i32, val: i32) -> Self {
        Self {
            key,
            val,
            prev: None, 
            next: None
        }
    }
}

#[derive(Default)]
struct List {
    head: Option<Rc<RefCell<ListNode>>>,
    tail: Option<Rc<RefCell<ListNode>>>,
}

impl List {
    fn new() -> Self {
        Self {
            head: None,
            tail: None
        }
    }

    fn push_back(&mut self, node: Option<Rc<RefCell<ListNode>>>) {
        if let Some(t) = self.tail.take() {
            if let Some(n) = &node {
                t.borrow_mut().next = Some(n.clone());
                n.borrow_mut().prev = Some(t);
            }
        } else {
            self.head = node.clone();
        }
        self.tail = node;
    }

    fn pop_front(&mut self) -> Option<Rc<RefCell<ListNode>>> {
        if let Some(h) = self.head.take() {
            if let Some(n) = h.borrow_mut().next.take() {
                n.borrow_mut().prev = None;
                self.head = Some(n);
            } else {
                self.head = None;
                self.tail = None;
            }

            Some(h)
        } else {
            None
        }
    }

    // 
    fn unlink_node(&mut self, node: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
        if let Some(n) = node {
            let prev = n.borrow_mut().prev.take();
            let next = n.borrow_mut().next.take();

            if let Some(p) = &prev {
                p.borrow_mut().next = next.clone();
            } else {
                self.head = next.clone();
            }

            if let Some(n) = &next {
                n.borrow_mut().prev = prev;
            } else {
                self.tail = prev;
            }
            
            Some(n)
        } else {
            None
        }
    }
}


struct LRUCache {
    cap: usize,
    used: usize,
    data: HashMap<i32, Rc<RefCell<ListNode>>>,
    list: List
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            cap: capacity as usize,
            used: 0,
            data: HashMap::new(),
            list: List::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.data.get(&key) {
            let val = node.borrow().val;

            let node = self.list.unlink_node(Some(node.clone()));
            self.list.push_back(node);
            val
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, val: i32) {
        if let Some(node) = self.data.get(&key) {
            node.borrow_mut().val = val;

            let node = self.list.unlink_node(Some(node.clone()));
            self.list.push_back(node);
        } else {
            if self.used == self.cap {
                if let Some(node) = self.list.pop_front() {
                    self.data.remove(&node.borrow().key);
                    self.used -= 1;
                }
            }

            let new_node = Rc::new(RefCell::new(ListNode::new(key, val)));
            self.data.insert(key, new_node.clone());
            self.list.push_back(Some(new_node));
            self.used += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn main_case() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(1, 2);
        assert_eq!(cache.get(1), 2);
        cache.put(2, 2);
        cache.put(3, 3);
        assert_eq!(cache.get(1), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(2), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }
}

fn main() {
    println!("Hello, world!");
}
