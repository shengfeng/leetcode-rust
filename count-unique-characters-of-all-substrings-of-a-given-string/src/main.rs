pub fn unique_letter_string(s: String) -> i32 {

}

#[cfg(test)]
mod tests {
    use crate::unique_letter_string;

    #[test]
    fn test_01() {
        let s = String::from("ABC");
        let ret = unique_letter_string(s);
        assert_eq!(ret, 10);
    }

    #[test]
    fn test_02() {
        let s = String::from("ABA");
        let ret = unique_letter_string(s);
        assert_eq!(ret, 8);
    }
}

fn main() {
    println!("Hello, world!");
}
