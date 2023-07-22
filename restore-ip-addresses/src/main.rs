pub fn restore_ip_addresses(s: String) -> Vec<String> {
    let mut result = vec![];
    let mut path = vec![];
    backtrack(&s, &mut path, &mut result);
    result
}

fn backtrack(s: &str, path: &mut Vec<String>, result: &mut Vec<String>) {
    if path.len() == 4 && s.is_empty() {
        result.push(path.join("."));
        return;
    }

    if path.len() >= 4 {
        return;
    }

    for i in 1..=3.min(s.len()) {
        let segments = &s[..i];
        if is_valid_segments(segments) {
            path.push(segments.to_string());
            backtrack(&s[i..], path, result);
            path.pop();
        }
    }
}

fn is_valid_segments(segment: &str) -> bool {
    if segment.starts_with("0") && segment != "0" {
        return false;
    }

    if let Ok(num) = segment.parse::<u8>() {
        return num <= 255;
    }

    false
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use crate::restore_ip_addresses;

    #[test]
    fn test_01() {
        let s = String::from("25525511135");
        let ret = restore_ip_addresses(s);
        let ok_ret = vec![String::from("255.255.11.135"), String::from("255.255.111.35")];
        assert_eq!(ret, ok_ret);
    }

    #[test]
    fn test_02() {
        let s = String::from("0000");
        let ret = restore_ip_addresses(s);
        let ok_ret = vec![String::from("0.0.0.0")];
        assert_eq!(ret, ok_ret);
    }

    #[test]
    fn test_03() {
        let s = String::from("101023");
        let ret = restore_ip_addresses(s);
        let ok_ret = vec![
            String::from("1.0.10.23"),
            String::from("1.0.102.3"),
            String::from("10.1.0.23"),
            String::from("10.10.2.3"),
            String::from("101.0.2.3")
        ];
        assert_eq!(ret, ok_ret);
    }
}