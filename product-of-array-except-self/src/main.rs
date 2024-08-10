fn main() {
    println!("Hello, world!");
}

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut prev = vec![1; n];
    let mut suffix = vec![1; n];

    for i in 1..n {
        prev[i] = prev[i - 1] * nums[i - 1];
    }

    for i in (0..n-1).rev() {
        suffix[i] = suffix[i + 1] * nums[i + 1];
    }

    prev.iter().zip(suffix.iter()).map(|(&p, &s)| p * s).collect()
}

#[cfg(test)]
mod tests {
    use crate::product_except_self;

    #[test]
    fn test_01() {
        let nums = vec![1, 2, 3, 4];
        let ret = vec![24, 12, 8, 6];
        assert_eq!(product_except_self(nums), ret);
    }

    #[test]
    fn test_02() {
        let nums = vec![-1, 1, 0, -3, 3];
        let ret = vec![0, 0, 9, 0, 0];
        assert_eq!(product_except_self(nums), ret);
    }
}