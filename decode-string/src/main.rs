pub fn decode_string(s: String) -> String {
    let mut stack = vec![];
    let mut res = String::new();
    let mut n = 0;

    for c in s.chars() {
        match c {
            '[' => {
                stack.push((n, res.clone()));
                n = 0;
                res.clear();
            },
            ']' => {
                if let Some(last) = stack.pop() {
                    res = last.1 + res.repeat(last.0).as_str();
                }
            },
            '0'..='9' => {
                n = n * 10 + (c as u8 - b'0') as usize;
            },
            _ => res.push(c)
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::decode_string;

    #[test]
    fn test_01() {
        let s = "3[a]2[bc]".to_string();
        let ret = "aaabcbc".to_string();
        assert_eq!(decode_string(s), ret);
    }

    #[test]
    fn test_02() {
        let s = "3[a2[c]]".to_string();
        let ret = "accaccacc".to_string();
        assert_eq!(decode_string(s), ret);
    }
    
    #[test]
    fn test_03() {
        let s = "2[abc]3[cd]ef".to_string();
        let ret = "abcabccdcdcdef".to_string();
        assert_eq!(decode_string(s), ret);
    }

    #[test]
    fn test_04() {
        let s = "abc3[cd]xyz".to_string();
        let ret = "abccdcdcdxyz".to_string();
        assert_eq!(decode_string(s), ret);
    }
}

fn main() {
    println!("Hello, world!");
}
