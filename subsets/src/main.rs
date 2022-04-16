pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    for mask in 0..(1 << nums.len()) {
        let mut t = vec![];
        for i in 0..nums.len() {
            if mask & (1 << i) != 0 {
                t.push(nums[i]);
            }
        }
        ret.push(t);
    }
    ret
}

mod tests {
    use crate::subsets;

    #[test]
    fn test_01() {
        let nums = vec![1,2,3];
        let ret = vec![
            vec![], vec![1], vec![2], vec![1,2], 
            vec![3], vec![1,3],vec![2,3], vec![1,2,3]
        ];
        assert_eq!(subsets(nums), ret);
    }

    #[test]
    fn test_02() {
        let nums = vec![0];
        let ret = vec![vec![], vec![0]];
        assert_eq!(subsets(nums), ret);
    }
}

fn main() {
    println!("Hello, world!");
}
