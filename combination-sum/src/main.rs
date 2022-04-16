pub fn backtrack(
    candidates: &Vec<i32>, start: usize, target: i32, 
    sum: i32, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if sum > target {
        return;
    }
    if sum == target {
        result.push(path.to_vec());
        return;
    }
    for i in start..candidates.len() {
        path.push(candidates[i]);
        backtrack(candidates, i, target, sum + candidates[i], path, result);
        path.pop();
    }
}

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut path = vec![];
    let mut result = vec![];
    backtrack(&candidates, 0, target, 0, &mut path, &mut result);
    result
}


mod test {
    use crate::combination_sum;

    #[test]
    fn test_01() {
        let candidates = vec![2,3,6,7];
        let ret = vec![vec![2,2,3], vec![7]];
        assert_eq!(combination_sum(candidates, 7), ret);
    }

    #[test]
    fn test_02() {
        let candidates = vec![2,3,5];
        let ret = vec![vec![2,2,2,2], vec![2,3,3], vec![3,5]];
        assert_eq!(combination_sum(candidates, 8), ret);
    }

    #[test]
    fn test_03() {
        let candidates = vec![2];
        let ret: Vec<Vec<i32>> = vec![];
        assert_eq!(combination_sum(candidates, 1), ret);
    }
}


fn main() {
    println!("Hello, world!");
}
