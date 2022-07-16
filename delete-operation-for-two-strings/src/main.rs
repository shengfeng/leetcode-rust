// dp[i][j] = dp[i-1][j-1] + 1, if word1[i-1] == word2[j-1]
// dp[i][j] = max(dp[i-1][j], dp[i][j-1]), if word[i-1] != word2[j-1]
pub fn min_distance(word1: String, word2: String) -> i32 {
    let m = word1.len();
    let n = word2.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            if word1.as_bytes()[i-1] == word2.as_bytes()[j-1] {
                dp[i][j] = dp[i-1][j-1] + 1;
            } else {
                dp[i][j] = std::cmp::max(dp[i-1][j], dp[i][j-1]);
            }
        }
    }
    let delete_num = dp[m][n];
    (m + n) as i32 - delete_num * 2
}

mod tests {
    use crate::min_distance;

    #[test]
    fn test_01() {
        let word1 = String::from("sea");
        let word2 = String::from("eat");
        assert_eq!(min_distance(word1, word2), 2);
    }

    #[test]
    fn test_02() {
        let word1 = String::from("leetcode");
        let word2 = String::from("etco");
        assert_eq!(min_distance(word1, word2), 4);
    }
}


fn main() {
    println!("Hello, world!");
}
