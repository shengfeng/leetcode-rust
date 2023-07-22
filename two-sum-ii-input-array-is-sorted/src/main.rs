

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = numbers.len() - 1;
    while left < right {
        if numbers[left] + numbers[right] == target {
            return vec![left as i32 + 1, right as i32 + 1];
        } else if numbers[left] + numbers[right] > target {
            right -= 1;
        } else {
            left += 1;
        }
    }
    vec![]
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::two_sum;


    #[test]
    fn test_01() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        let ret = two_sum(numbers, target);
        assert_eq!(ret, vec![1, 2]);
    }

    #[test]
    fn test_02() {
        let numbers = vec![2, 3, 4];
        let target = 6;
        let ret = two_sum(numbers, target);
        assert_eq!(ret, vec![1, 3]);
    }

    #[test]
    fn test_03() {
        let numbers = vec![-1, 0];
        let target = -1;
        let ret = two_sum(numbers, target);
        assert_eq!(ret, vec![1, 2]);
    }
}
