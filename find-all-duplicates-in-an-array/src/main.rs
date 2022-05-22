pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![];
    
    for i in 0..nums.len() {
        let idx = nums[i].abs() - 1;
        let num = nums.get_mut(idx as usize).unwrap();
        if *num < 0 {
            ret.push(idx as i32 + 1);
        } else {
            *num *= -1;
        }
    }
    
    ret
}

#[cfg(test)]
mod tests {
    use crate::find_duplicates;

    #[test]
    fn test_01() {
        let nums = vec![4,3,2,7,8,2,3,1];
        let ret = vec![2, 3];
        assert_eq!(find_duplicates(nums), ret);
    }

    #[test]
    fn test_02() {
        let nums = vec![1,1,2];
        let ret = vec![1];
        assert_eq!(find_duplicates(nums), ret);
    }

    #[test]
    fn test_03() {
        let nums = vec![1];
        assert_eq!(find_duplicates(nums), vec![]);
    }
}

fn main() {
    println!("Hello, world!");
}
