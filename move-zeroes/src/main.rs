pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut last_index = 0;
    for fast_index in 0..nums.len() {
        if nums[fast_index] != 0 {
            nums[last_index] = nums[fast_index];
            last_index += 1;
        }
    }

    while last_index < nums.len() {
        nums[last_index] = 0;
        last_index += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::move_zeroes;

    #[test]
    fn test_01() {
        let mut nums = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn test_02() {
        let mut nums = vec![0];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }
}

fn main() {
    println!("Hello, world!");
}
