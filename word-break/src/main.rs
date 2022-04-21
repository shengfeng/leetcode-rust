// dp[i] represents the first i of the s can be separated
// dp[i]=dp[j] && check(s[j..i−1])
pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut word_dict = word_dict;
    word_dict.sort();
    let mut dp = vec![false; s.len() + 1];
    dp[0] = true;
    for i in 1..=s.len() {
        for j in 0..i {
            if dp[j] && word_dict.contains(&s[j..i].to_string()) {
                dp[i] = true;
                break;
            }
        }
    }
    println!("{:?}", dp);
    dp[s.len()]
}


mod tests {
    use crate::word_break;

    #[test]
    fn test_01() {
        let s = String::from("leetcode");
        let word_dict = vec!["leet".to_string(), "code".to_string()];
        assert_eq!(word_break(s, word_dict), true);
    }

    #[test]
    fn test_02() {
        let s = String::from("applepenapple");
        let word_dict = vec!["apple".to_string(), "pen".to_string()];
        assert_eq!(word_break(s, word_dict), true);
    }

}

fn main() {
    let s = String::from("leetcode");
    let word_dict = vec!["leet".to_string(), "code".to_string()];
    let ret = word_break(s, word_dict);
}
