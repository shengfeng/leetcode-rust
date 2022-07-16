// dp[i] = max(dp[i], dp[i-j]*j), j \in 2..i
pub fn integer_break(n: i32) -> i32 {
    let mut dp = vec![0i32; n as usize + 1];
    dp[1] = 1;
    dp[2] = 1;
    for i in 3..=n as usize {
        for j in 1..i {
            dp[i] = std::cmp::max(
                dp[i].max(((i - j) * j) as i32), 
                dp[i - j] * j as i32);
        }
    }
    println!("{:?}", dp);
    dp[n as usize]
}

mod tests {
    use crate::integer_break;

    #[test]
    fn test_01() {
        assert_eq!(integer_break(2), 1);
    }

    #[test]
    fn test_02() {
        assert_eq!(integer_break(10), 36);
    }
}

fn main() {
    let ret = integer_break(4);
}
