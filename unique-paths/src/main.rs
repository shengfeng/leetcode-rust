// dp[i][j] = dp[i - 1][j] + dp[i][j - 1]
pub fn unique_paths(m: i32, n: i32) -> i32 {
    let mut dp = vec![vec![0; n as usize]; m as usize];
    for i in 0..m as usize {
        dp[i][0] = 1;
    }
    for j in 0..n as usize {
        dp[0][j] = 1;
    }
    for i in 1..m as usize {
        for j in 1..n as usize {
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
        }
    }
    dp[(m - 1) as usize][(n - 1) as usize]
}

mod tests {
    use crate::unique_paths;

    #[test]
    fn test_01() {
        assert_eq!(unique_paths(3, 7), 28);
    }

    #[test]
    fn test_02() {
        assert_eq!(unique_paths(3, 2), 3);
    }
}

fn main() {
    unique_paths(3, 2);
}
