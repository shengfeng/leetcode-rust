fn main() {
    println!("Hello, world!");
}

pub fn reverse_words(s: String) -> String {
    let mut ret = String::new();

    for word in s.split_whitespace().rev() {
        ret.push_str(word);
        ret.push(' ');
    }
    ret.pop();
    ret
}

#[cfg(test)]
mod tests {
    use crate::reverse_words;

    #[test]
    fn test_01() {
        let s = String::from("the sky is blue");
        let ret = String::from("blue is sky the");
        assert_eq!(reverse_words(s), ret);
    }

    #[test]
    fn test_02() {
        let s = String::from("  hello world  ");
        let ret = String::from("world hello");
        assert_eq!(reverse_words(s), ret);
    }

    #[test]
    fn test_03() {
        let s = String::from("a good   example");
        let ret = String::from("example good a");
        assert_eq!(reverse_words(s), ret);
    }
}