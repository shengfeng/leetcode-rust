/* 
 * s[i] == 0: dp[i] = dp[i - 1]
 * s[i - 1] == '1': dp[i] = dp[i - 1] + dp[i - 2]
 * s[i - 1] == '2' && '1' <= s[i] <= '6', dp[i] = dp[i - 1] + dp[i - 2] 
 */
pub fn num_decodings(s: String) -> i32 {
    let mut dp = vec![0; s.len()+1];
    dp[0] = 1;
    let words = s.as_bytes().to_vec();
    for i in 1..=words.len() {
        // if s[i] != 0, dp[i] = dp[i-1]
        if words[i - 1] != b'0' {
            dp[i] = dp[i - 1];
        }
        if i > 1 && words[i - 2] != b'0' {
            let a = (words[i - 2] - b'0') * 10 + (words[i - 1] - b'0');
            println!("a = {}", a);
            if a <= 26 {
                dp[i] += dp[i - 2]
            }
        }
    }
    println!("{:?}", dp);
    dp[words.len()]
}

mod tests {
    use crate::num_decodings;

    #[test]
    fn test_01() {
        assert_eq!(num_decodings(String::from("12")), 2);
    }

    #[test]
    fn test_02() {
        assert_eq!(num_decodings(String::from("226")), 3);
    }


    #[test]
    fn test_03() {
        assert_eq!(num_decodings(String::from("0")), 0);
    }
}

fn main() {
    // 226 --> BBF(2, 2, 6), BZ(2, 26), VF(22, 6)
    let ret = num_decodings(String::from("10"));
}
