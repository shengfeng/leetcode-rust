pub fn jump(nums: Vec<i32>) -> i32 {
    let mut max_far = 0;
    let mut end = 0;
    let mut step = 0;
    for i in 0..nums.len()-1 {
        max_far = max_far.max(i + nums[i] as usize);
        if i == end {
            println!("{:?}", max_far);
            end = max_far;
            step += 1;
        }
    }
    step
}

mod tests {
    use crate::jump;

    #[test]
    fn test_01() {
        let nums = vec![2,3,1,1,4];
        assert_eq!(jump(nums), 2);
    }

    #[test]
    fn test_02() {
        let nums = vec![2,3,0,1,4];
        assert_eq!(jump(nums), 2);
    }
}

fn main() {
    let nums = vec![2,3,1,1,4];
    let ret = jump(nums);
}
