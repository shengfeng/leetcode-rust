// dp[i][j] = dp[i+1][j-1] /\ s[i] == s[j]
pub fn longest_palindrome(s: String) -> String {
    if s.len() < 1 {
        return String::new();
    }
    let mut start = 0;
    let mut end = 0;
    let word = s.as_bytes().to_vec();
    for i in 0..s.len() {
        let len1 = expend_around_center(&word, i, i);
        let len2 = expend_around_center(&word, i, i + 1);
        let len = len1.max(len2);
        if len > end - start {
            start = i - (len - 1) / 2;
            end = i + len / 2;
        }
    }
    s[start..=end].to_string()
}


pub fn expend_around_center(s: &Vec<u8>, left: usize, right: usize) -> usize {
    let mut l = left as i32;
    let mut r = right as i32;
    while l >= 0 && r < s.len() as i32 && s[l as usize] == s[r as usize] {
        l -= 1;
        r += 1
    }
    (r - l - 1) as usize
}

mod tests {
    use crate::longest_palindrome;

    #[test]
    fn test_01() {
        let s = String::from("babad");
        assert_eq!(longest_palindrome(s), String::from("aba"));
    }

    #[test]
    fn test_02() {
        let s = String::from("cbbd");
        assert_eq!(longest_palindrome(s), String::from("bb"));
    }
}


fn main() {
    println!("Hello world!");
}