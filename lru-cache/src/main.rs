use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

struct Node {
    key: i32,
    val: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32, val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { key, val, prev: None, next: None }))
    }
}

struct LRUCache {
    capacity: usize,
    dummy: Rc<RefCell<Node>>,
    key_to_node: HashMap<i32, Rc<RefCell<Node>>>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let dummy = Node::new(0, 0);
        dummy.borrow_mut().prev = Some(dummy.clone());
        dummy.borrow_mut().next = Some(dummy.clone());
        LRUCache { capacity: capacity as usize, dummy, key_to_node: HashMap::new() }
    }

    fn remove(&mut self, x: Rc<RefCell<Node>>) {
        let prev = x.borrow().prev.clone().unwrap();
        let next = x.borrow().next.clone().unwrap();
        prev.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(prev);
    }

    fn push_front(&mut self, x: Rc<RefCell<Node>>) {
        /*
            dummy <--> A 
            ------------
            dummy <--> x <--> A
         */
        let next = self.dummy.borrow().next.clone();
        x.borrow_mut().prev = Some(self.dummy.clone());
        x.borrow_mut().next = next.clone();
        self.dummy.borrow_mut().next = Some(x.clone());
        next.unwrap().borrow_mut().prev = Some(x);
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.key_to_node.get(&key) {
            let node = node.clone();
            let value = node.borrow().val;
            self.remove(node.clone());
            self.push_front(node);
            return value;
        }
        -1
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.key_to_node.get(&key) {
            let node = node.clone();
            node.borrow_mut().val = value;
            self.remove(node.clone());
            self.push_front(node);
            return;
        }

        let node = Node::new(key, value);
        self.key_to_node.insert(key, node.clone());
        self.push_front(node);

        if self.key_to_node.len() > self.capacity {
            let back_node = self.dummy.borrow().prev.clone().unwrap();
            self.key_to_node.remove(&back_node.borrow().key);
            self.remove(back_node);
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
