// dp[i][j] = dp[i - 1][j - 1] + 1 if text1[i - 1] == text[j - 1]
// dp[i][j] = max(dp[i-1][j], dp[i][j-1]) if text1[i - 1] != text[j - 1]
pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let m = text1.len();
    let n = text2.len();
    let mut dp = vec![vec![0; n+1]; m+1];
    for i in 1..=m {
        for j in 1..=n {
            if text1.as_bytes()[i - 1] == text2.as_bytes()[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }
    dp[m][n]
}


mod tests {
    use crate::longest_common_subsequence;

    #[test]
    fn test_01() {
        let text1 = String::from("abcde");
        let text2 = String::from("ace");
        assert_eq!(longest_common_subsequence(text1, text2), 3);
    }

    #[test]
    fn test_02() {
        let text1 = String::from("abc");
        let text2 = String::from("abc");
        assert_eq!(longest_common_subsequence(text1, text2), 3);
    }

    #[test]
    fn test_03() {
        let text1 = String::from("abc");
        let text2 = String::from("def");
        assert_eq!(longest_common_subsequence(text1, text2), 0);
    }
}



fn main() {
    println!("Hello, world!");
}
