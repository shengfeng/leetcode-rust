pub fn backtrack(nums: &Vec<i32>, start: usize, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, used: &mut Vec<bool>) {
    result.push(path.to_vec());
    for i in start..nums.len() {
        if i > start && nums[i] == nums[i - 1] && !used[i] {
            continue;
        }
        path.push(nums[i]);
        used[i] = true;
        backtrack(nums, i + 1, path, result, used);
        used[i] = false;
        path.pop();
    }
}   

pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut path = vec![];
    let mut result = vec![];
    let mut used = vec![false; nums.len()];
    let mut nums = nums;
    nums.sort();
    backtrack(&nums, 0, &mut path, &mut result, &mut used);
    result
}

mod tests {
    use crate::subsets_with_dup;

    #[test]
    fn test_01()  {
        let nums = vec![1,2,2];
        let ret = vec![
            vec![], vec![1], vec![1,2], vec![1,2,2],
            vec![2], vec![2,2]
        ];
        assert_eq!(subsets_with_dup(nums), ret);
    }

    #[test]
    fn test_02() {
        let nums = vec![0];
        let ret = vec![vec![], vec![0]];
        assert_eq!(subsets_with_dup(nums), ret);
    }
}

fn main() {
    println!("Hello, world!");
}
