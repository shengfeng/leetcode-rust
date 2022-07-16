// 1. exhaustive 
pub fn is_arithmetic(nums: &Vec<i32>, start: usize, end: usize) -> bool {
    if end - start < 2 {
        return false;
    }
    for i in start..end - 1 {
        if nums[i + 1] * 2 != nums[i] + nums[i + 2] {
            return false;
        }
    }
    return true;
}   


pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    if nums.len() < 3 {
        return count;
    }

    for i in 0..nums.len() - 2 {
        for j in i+1..nums.len() {
            if is_arithmetic(&nums, i, j) {
                count += 1;
            }
        }
    }
    count
}

// 2. sliding window
pub fn number_of_arithmetic_slices2(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    if nums.len() < 3 {
        return count;
    }
    for i in 0..nums.len() - 2 {
        let d = nums[i + 1] - nums[i];
        for j in i+1..nums.len()-1 {
            if nums[j+1] - nums[j] == d {
                count += 1;
            } else {
                break;
            }
        } 
    }
    count
}

// recursive
pub fn number_of_arithmetic_slices3(nums: Vec<i32>) -> i32 {
    let mut dp = vec![0; nums.len()];
    let mut ret = 0;
    for i in 2..nums.len() {
        if nums[i] - nums[i - 1] == nums[i - 1] - nums[i - 2] {
            dp[i] = dp[i - 1] + 1;
            ret += dp[i];
        }
    }
    ret
}

mod tests {
    use crate::*;

    #[test]
    fn test_01() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(number_of_arithmetic_slices(nums), 3);
    }

    #[test]
    fn test_02() {
        let nums = vec![1];
        assert_eq!(number_of_arithmetic_slices(nums), 0);
    }

    #[test]
    fn test_03() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(number_of_arithmetic_slices2(nums), 3);
    }

    #[test]
    fn test_04() {
        let nums = vec![1];
        assert_eq!(number_of_arithmetic_slices2(nums), 0);
    }

    #[test]
    fn test_05() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(number_of_arithmetic_slices3(nums), 3);
    }

    #[test]
    fn test_06() {
        let nums = vec![1];
        assert_eq!(number_of_arithmetic_slices3(nums), 0);
    }
}

fn main() {
    println!("Hello, world!");
}
