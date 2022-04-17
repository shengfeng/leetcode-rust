
pub fn can_jump(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let mut most_right = 0;
    for i in 0..n {
        if i <= most_right {
            let candidate = nums[i] as usize + i;
            most_right = most_right.max(candidate);
            if most_right >= n - 1 {
                return true;
            }
        }
    }
    false
}

mod tests {
    use crate::can_jump;

    #[test]
    fn test_01() {
        let nums = vec![2, 3, 1, 1, 4];
        assert_eq!(can_jump(nums), true);
    }

    #[test]
    fn test_02() {
        let nums = vec![3,2,1,0,4];
        assert_eq!(can_jump(nums), false);
    }

    #[test]
    fn test_03() {
        let nums = vec![0];
        assert_eq!(can_jump(nums), true);
    }
}

fn main() {
    println!("Hello, world!");
}
