pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    if k <= 1 {
        return 0;
    }

    let mut count = 0;
    let mut left = 0;
    let mut r = 1;
    for (right, val)in nums.iter().enumerate() {
        println!("right = {:?}, val = {:?}", right, val);
        r *= val;
        while r >= k {
            r = r / nums[left];
            left += 1
        }
        count += right - left + 1;
    }
    count as i32
}


#[test]
fn test_01() {
    let nums = vec![10, 5, 2, 6];
    let k = 100;
    assert_eq!(num_subarray_product_less_than_k(nums, k), 8);
}

#[test]
fn test_02() {
    let nums = vec![1, 2, 3];
    let k = 0;
    assert_eq!(num_subarray_product_less_than_k(nums, k), 0);
}

fn main() {
    println!("Hello, world!");
}
