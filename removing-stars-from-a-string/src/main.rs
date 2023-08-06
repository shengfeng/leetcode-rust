pub fn remove_stars(s: String) -> String {
    let mut ret = vec![];
    for c in s.chars() {
        match c {
            '*' => {
                ret.pop();
            },
            _ => {
                ret.push(c);
            }
        }
    }
    ret.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::remove_stars;

    #[test]
    fn test_01() {
        let s = "leet**cod*e".to_string();
        let ret = "lecoe".to_string();
        assert_eq!(remove_stars(s), ret);
    }

    #[test]
    fn test_02() {
        let s = "erase*****".to_string();
        let ret = "".to_string();
        assert_eq!(remove_stars(s), ret);
    }
}


fn main() {
    println!("Hello, world!");
}
