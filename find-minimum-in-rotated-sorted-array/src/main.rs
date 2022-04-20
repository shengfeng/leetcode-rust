pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] < nums[right] {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    nums[left]
}

#[test]
fn test_01() {
    let nums = vec![11, 13, 15, 17];
    let ret = find_min(nums);
    assert_eq!(ret, 11);
}

#[test]
fn test_02() {
    let nums = vec![4,5,6,7,0,1,2];
    let ret = find_min(nums);
    assert_eq!(ret, 0);
}

#[test]
fn test_03() {
    let nums = vec![3,4,5,1,2];
    let ret = find_min(nums);
    assert_eq!(ret, 1);
}


fn main() {
    let nums = vec![11, 13, 15, 17];
    let ret = find_min(nums);
    println!("{:#?}", ret);
}
