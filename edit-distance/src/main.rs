// 定义：dp[i][j] 代表 word1 中前 i 个字符，变换到 word2 中前 j 个字符，最短需要操作的次数;
// 状态转移：
//    1. 增, dp[i][j] = dp[i][j - 1] + 1
//    2. 删, dp[i][j] = dp[i - 1][j] + 1
//    3. 改, dp[i][j] = dp[i - 1][j - 1] + 1
// 初始状态：
//     dp[i][0] = i, dp[0][j] = j
pub fn min_distance(word1: String, word2: String) -> i32 {
    let m = word1.len();
    let n = word2.len();
    let mut dp = vec![vec![0; n+1]; m+1];
    for i in 1..=m {
        dp[i][0] = i as i32;
    }
    for j in 1..=n {
        dp[0][j] = j as i32;
    }
    for i in 1..=m {
        for j in 1..=n {
            if word1.as_bytes()[i-1] == word2.as_bytes()[j-1] {
                dp[i][j] = dp[i-1][j-1];
            } else {
                dp[i][j] = std::cmp::min(
                    dp[i][j-1], 
                    dp[i-1][j].min(dp[i-1][j-1])) + 1;
            }
        }
    }
    dp[m][n]
}


mod tests {
    use crate::min_distance;

    #[test]
    fn test_01() {
        let word1 = String::from("horse");
        let word2 = String::from("ros");
        assert_eq!(min_distance(word1, word2), 3);
    }

    #[test]
    fn test_02() {
        let word1 = String::from("intention");
        let word2 = String::from("execution");
        assert_eq!(min_distance(word1, word2),5);
    }
}

fn main() {
    println!("Hello, world!");
}
