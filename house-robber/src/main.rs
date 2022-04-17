
// dp[i] = max(dp[i-1], dp[i-2] + nums[i])
pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    let mut dp = vec![0; nums.len()];
    dp[0] = nums[0];
    dp[1] = std::cmp::max(nums[0], nums[1]);
    for i in 2..nums.len() {
        dp[i] = std::cmp::max(dp[i-1], dp[i-2] + nums[i]);
    }
    dp[nums.len()-1]
}

mod tests {
    use crate::rob;

    #[test]
    fn test_01() {
        let nums = vec![1,2,3,1];
        assert_eq!(rob(nums), 4);
    }

    #[test]
    fn test_02() {
        let nums = vec![2,7,9,3,1];
        assert_eq!(rob(nums), 12);
    }

    #[test]
    fn test_03() {
        let nums = vec![0];
        assert_eq!(rob(nums), 0);
    }
}

fn main() {
    println!("Hello, world!");
}
