pub fn remove_outer_parentheses(s: String) -> String {
    use std::collections::VecDeque;

    let mut ret = String::from("");
    let mut stack = VecDeque::new();
    s.bytes().into_iter().for_each(|c| {
        if c == b')' {
            stack.pop_back();
        }
        if !stack.is_empty() {
            ret.push(char::from(c));
        }
        if c == b'(' {
            stack.push_back(c);
        }
    });
    ret
}


#[cfg(test)]
mod tests {
    use crate::remove_outer_parentheses;

    #[test]
    fn test_01() {
        let s = String::from("(()())(())");
        let r = String::from("()()()");
        assert_eq!(remove_outer_parentheses(s), r);
    }

    #[test]
    fn test_02() {
        let s = String::from("(()())(())(()(()))");
        let r = String::from("()()()()(())");
        assert_eq!(remove_outer_parentheses(s), r);
    }


    #[test]
    fn test_03() {
        let s = String::from("()()");
        let r = String::from("");
        assert_eq!(remove_outer_parentheses(s), r);
    }
}


fn main() {
    println!("Hello, world!");
}
