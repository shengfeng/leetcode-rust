
pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    if nums.len() == 1 {
        return nums[0];
    }
    let result1 = rob_range(&nums, 0, nums.len() - 2);
    let result2 = rob_range(&nums, 1, nums.len() - 1);
    std::cmp::max(result1, result2)
}

// dp[i] = max(dp[i - 1], dp[i - 2] + nums[i])
pub fn rob_range(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
    if start == end {
        return nums[start];
    }
    let mut dp = vec![0; nums.len()];
    dp[start] = nums[start];
    dp[start+1] = std::cmp::max(nums[start], nums[start+1]);
    for i in start+2..=end {
        dp[i] = std::cmp::max(dp[i - 1], dp[i - 2] + nums[i]);
    }
    dp[end]
}

mod tests {
    use crate::rob;

    #[test]
    fn test_01() {
        let nums = vec![2, 3, 2];
        assert_eq!(rob(nums), 3);
    }


    #[test]
    fn test_02() {
        let nums = vec![1, 2, 3, 2];
        assert_eq!(rob(nums), 4);
    }

    
    #[test]
    fn test_03() {
        let nums = vec![1, 2, 3];
        assert_eq!(rob(nums), 3);
    }
}

fn main() {
    println!("Hello, world!");
}
