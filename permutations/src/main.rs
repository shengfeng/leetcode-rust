pub fn backtrack(nums: &Vec<i32>, n: usize, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, used: &mut Vec<bool>) {
    if path.len() == n {
        result.push(path.to_vec());
        return;
    }
    for i in 0..nums.len() {
        if used[i] {
            continue;
        }
        used[i] = true;
        path.push(nums[i]);
        backtrack(nums, n, path, result, used);
        path.pop();
        used[i] = false;
    }
}


pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut path = vec![];
    let mut result = vec![];
    let n = nums.len();
    let mut used = vec![false; n];
    backtrack(&nums, n, &mut path, &mut result, &mut used);
    result
}

mod tests {
    use crate::permute;

    #[test]
    fn test_01() {
        let nums = vec![1,2,3];
        let ret = vec![
            vec![1,2,3], vec![1,3,2], vec![2,1,3],
            vec![2,3,1], vec![3,1,2], vec![3,2,1]
        ];
        assert_eq!(permute(nums), ret);
    }

    #[test]
    fn test_02() {
        let nums = vec![0, 1];
        let ret = vec![vec![0,1], vec![1,0]];
        assert_eq!(permute(nums), ret);
    }

    #[test]
    fn test_03() {
        let nums = vec![1];
        let ret = vec![vec![1]];
        assert_eq!(permute(nums), ret);
    }
}

fn main() {
    println!("Hello, world!");
}
