use std::collections::VecDeque;

struct MovingAverage {
    store: VecDeque<i32>,
    size: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {

    /** Initialize your data structure here. */
    fn new(size: i32) -> Self {
        MovingAverage {
            store: VecDeque::new(),
            size: size as usize,
        }
    }
    
    fn next(&mut self, val: i32) -> f64 {
        if self.store.len() == self.size {
            self.store.pop_front();
        }
        self.store.push_back(val);
        self.store.iter().sum::<i32>() as f64 / (self.store.len() as f64)
    }
}

#[cfg(test)]
mod tests{
    use crate::MovingAverage;

    #[test]
    fn test_01() {
        let mut moving_average = MovingAverage::new(3);
        assert_eq!(moving_average.next(1), 1.0);
        assert_eq!(moving_average.next(10), 5.5);
        assert_eq!(moving_average.next(3), 4.666666666666667);
        assert_eq!(moving_average.next(5), 6.0);
    }
}

fn main() {
    println!("Hello, world!");
}
