fn main() {
    println!("Hello, world!");
}

pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut sum = nums[0..(k as usize)].iter().sum::<i32>();
    let mut mean = sum as f64 / k as f64;

    for i in (k as usize)..nums.len() {
        sum = sum + nums[i] - nums[i - k as usize];
        mean = mean.max(sum as f64 / k as f64);
    }
    mean
}


#[cfg(test)]
mod tests {
    use crate::find_max_average;


    #[test]
    fn test_01(){
        let nums = vec![1, 12, -5, -6, 50, 3];
        assert_eq!(find_max_average(nums, 4), 12.75);
    }

    #[test]
    fn test_02() {
        let nums = vec![5];
        assert_eq!(find_max_average(nums, 1), 5.00);
    }
}
