pub fn is_subsequence(s: String, t: String) -> bool {
    let mut last_index = 0;
    let s_vec = s.as_bytes().to_vec();
    let t_vec = t.as_bytes().to_vec();

    if s_vec.len() == 0 {
        return true;
    }

    for fast_index in 0..t.len() {
        if s_vec[last_index] == t_vec[fast_index] {
            last_index += 1;
        }
        if last_index == s.len() {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::is_subsequence;

    #[test]
    fn test_01() {
        let s = "abc".to_string();
        let t = "ahbgdc".to_string();
        assert_eq!(is_subsequence(s, t), true);
    }

    #[test]
    fn test_02() {
        let s = "axc".to_string();
        let t = "ahbgdc".to_string();
        assert_eq!(is_subsequence(s, t), false);
    }
}

fn main() {
    println!("Hello, world!");
}
