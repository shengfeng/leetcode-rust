// dp[i] = max(dp[j]) + 1 if j < i && nums[j] < nums[i]
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut dp = vec![0; nums.len()];
    if nums.len() == 0 {
        return 0;
    }
    for i in 0..nums.len() {
        dp[i] = 1;
        for j in 0..i {
            if nums[j] < nums[i] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }
    *dp.iter().max().unwrap()
}


mod tests {
    use super::length_of_lis;

    #[test]
    fn test_01() {
        let nums = vec![10,9,2,5,3,7,101,18];
        assert_eq!(length_of_lis(nums), 4);
    }

    #[test]
    fn test_02() {
        let nums = vec![0,1,0,3,2,3];
        assert_eq!(length_of_lis(nums), 4);
    }

    #[test]
    fn test_03() {
        let nums = vec![7,7,7,7,7,7,7];
        assert_eq!(length_of_lis(nums), 1);
    }
}

fn main() {
    println!("Hello, world!");
}
