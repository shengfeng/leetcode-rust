use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(nums.len());
    for (index, num) in nums.iter().enumerate() {
        match map.get(&(target - num)) {
            None => {
                map.insert(num, index);
            },
            Some(sub_index) =>
                return vec![*sub_index as i32, index as i32]
        }
    }
    vec![]
}


mod tests {
    use crate::*;

    #[test]
    fn test_01() {
        let nums = vec![2,7,11,15];
        assert_eq!(two_sum(nums, 9), vec![0, 1]);
    }

    #[test]
    fn test_02() {
        let nums = vec![3,2,4];
        assert_eq!(two_sum(nums, 6), vec![1, 2]);
    }

    #[test]
    fn test_03() {
        let nums = vec![3,3];
        assert_eq!(two_sum(nums, 6), vec![0, 1]);
    }
}
