fn backtrack(nums: & Vec<i32>, used: &mut Vec<bool>, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if path.len() == nums.len() {
        result.push(path.to_vec());
        return;
    }

    for i in 0..nums.len() {
        if used[i] || (i > 0 && nums[i] == nums[i - 1] && !used[i - 1]) {
            continue;
        }
        path.push(nums[i]);
        used[i] = true;
        backtrack(nums, used, path, result);
        used[i] = false;
        path.pop();
    }
}

pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    let mut path: Vec<i32> = vec![];
    let mut used: Vec<bool> = vec![false; nums.len()];
    
    let mut sorted_nums = nums;
    sorted_nums.sort();
    backtrack(&sorted_nums, &mut used, &mut path, &mut result);
    result
}

mod tests {
    use crate::permute_unique;

    #[test]
    fn test_01() {
        let nums = vec![1, 1, 2];
        let ret = vec![vec![1,1,2], vec![1,2,1], vec![2,1,1]];
        assert_eq!(permute_unique(nums), ret);
    }

    #[test]
    fn test_02() {
        let nums = vec![1, 2, 3];
        let ret = vec![
            vec![1,2,3], vec![1,3,2], vec![2,1,3],
            vec![2,3,1], vec![3,1,2], vec![3,2,1]
        ];
        assert_eq!(permute_unique(nums), ret);
    }
}

fn main() {
    println!("Hello, world!");
}
