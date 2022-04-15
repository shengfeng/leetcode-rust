pub fn backtrack(
    candidates: &Vec<i32>, start: usize, target: i32, sum: i32, 
    path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if sum > target {
        return;
    }
    if sum == target {
        result.push(path.to_vec());
        return;
    }
    for i in start..candidates.len() {

        if i > start && candidates[i] == candidates[i - 1] {
            continue;
        }

        path.push(candidates[i]);
        backtrack(candidates, i + 1, target, sum + candidates[i], path, result);
        path.pop();
    }
}

pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut path = vec![];
    let mut result = vec![];
    let mut candidates = candidates;
    candidates.sort();
    backtrack(&candidates, 0, target, 0, &mut path, &mut result);
    result
}

mod tests {
    use crate::combination_sum2;

    #[test]
    fn test_01() {
        let candidates = vec![10,1,2,7,6,1,5];
        let ret = vec![
            vec![1,1,6], vec![1,2,5],
            vec![1,7], vec![2,6]
        ];
        assert_eq!(combination_sum2(candidates, 8), ret);
    }

    #[test]
    fn test_02() {
        let candidates = vec![2,5,2,1,2];
        let ret = vec![vec![1,2,2], vec![5]];
        assert_eq!(combination_sum2(candidates, 5), ret);
    }
}

fn main() {
    println!("Hello, world!");
}
