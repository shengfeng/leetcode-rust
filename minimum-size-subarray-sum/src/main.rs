pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut ret = i32::MAX;
    let mut sum = 0;
    let mut i = 0;
    for j in 0..nums.len() {
        sum += nums[j];
        while sum >= target {
            let sub_length = (j - i + 1) as i32;
            ret = ret.min(sub_length);
            sum -= nums[i];
            i += 1;
        }
    }
    if ret == i32::MAX {
        0
    } else {
        ret
    }
}

#[test]
fn test_01() {
    let nums = vec![2,3,1,2,4,3];
    let target = 7;
    assert_eq!(min_sub_array_len(target, nums), 2);
}

#[test]
fn test_02() {
    let nums = vec![1,4,4];
    let target = 4;
    assert_eq!(min_sub_array_len(target, nums), 1);
}

#[test]
fn test_03() {
    let nums = vec![1,1,1,1,1,1,1,1];
    let target = 11;
    assert_eq!(min_sub_array_len(target, nums), 0);
}
fn main() {
    println!("Hello, world!");
}
