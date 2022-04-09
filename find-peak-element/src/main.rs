pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] > nums[mid + 1] {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left as i32
}


#[test]
fn test_01() {
    let nums = vec![1, 2, 3, 1];
    let ret = find_peak_element(nums);
    assert_eq!(ret, 2);
}

#[test]
fn test_02() {
    let nums = vec![1, 2, 1, 3, 5, 6, 4];
    let ret = find_peak_element(nums);
    assert_eq!(ret, 5);
}


fn main() {
    println!("Hello, world!");
}
