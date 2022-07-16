// dp[i] = max(dp[j]) + 1, j < i && nums[j] < nums[i]
pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let mut dp = vec![0; nums.len()];
    let mut cnt = vec![0; nums.len()];
    let mut max_len = 0;
    let mut ret = 0;
    for i in 0..nums.len() {
        dp[i] = 1;
        cnt[i] = 1;
        for j in 0..i {
            if nums[j] < nums[i] {
                if dp[i] < dp[j] + 1 {
                    dp[i] = dp[j] + 1;
                    cnt[i] = cnt[j];
                } else if dp[i] == dp[j] + 1 {
                    // cnt[i] is the sum of cnt[j] that dp[i] == d[j] + 1
                    cnt[i] += cnt[j];
                }
            }
        }
        if dp[i] > max_len {
            max_len = dp[i];
            ret = cnt[i];
        } else if dp[i] == max_len {
            ret += cnt[i];
        }
    }
    ret
}

mod tests {
    use crate::find_number_of_lis;

    #[test]
    fn test_01() {
        let nums = vec![1,3,5,4,7];
        assert_eq!(find_number_of_lis(nums), 2);
    }

    #[test]
    fn test_02() {
        let nums = vec![2,2,2,2,2];
        assert_eq!(find_number_of_lis(nums), 5);
    }
}

fn main() {
    println!("Hello, world!");
}
