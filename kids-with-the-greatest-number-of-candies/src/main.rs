pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max = *candies.iter().max().unwrap_or(&0);
    candies.into_iter().map(|c| c + extra_candies >= max).collect()
}

#[cfg(test)]
mod tests {
    use crate::kids_with_candies;


    #[test]
    fn test_01() {
        let candies = vec![2, 3, 5, 1, 3];
        let extra_candies = 3;
        let ret = vec![true, true, true, false, true];
        assert_eq!(kids_with_candies(candies, extra_candies), ret);
    }

    #[test]
    fn test_02() {
        let candies = vec![4, 2, 1, 1, 2];
        let extra_candies = 1;
        let ret = vec![true, false, false, false, false];
        assert_eq!(kids_with_candies(candies, extra_candies), ret);
    }

    #[test]
    fn test_03() {
        let candies = vec![12, 1, 12];
        let extra_candies = 10;
        let ret = vec![true, false, true];
        assert_eq!(kids_with_candies(candies, extra_candies), ret);
    }
}

fn main() {
    println!("Hello, world!");
}
