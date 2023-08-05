pub fn gcd_of_strings(str1: String, str2: String) -> String {
    if str1.clone() + &str2 != str2.clone() + &str1 {
        return "".into();
    }

    let n = gcd(str1.len() as i32, str2.len() as i32) as usize;
    str1[0..n].to_string()
}

pub fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[cfg(test)]
mod tests {
    use crate::gcd_of_strings;

    #[test]
    fn test_01() {
        let str1 = "ABCABC".to_string();
        let str2 = "ABC".to_string();
        let ret = "ABC".to_string();
        assert_eq!(gcd_of_strings(str1, str2), ret);
    }

    #[test]
    fn test_02() {
        let str1 = "ABABAB".to_string();
        let str2 = "AB".to_string();
        let ret = "AB".to_string();
        assert_eq!(gcd_of_strings(str1, str2), ret);
    }

    #[test]
    fn test_03() {
        let str1 = "LEET".to_string();
        let str2 = "CODE".to_string();
        let ret = "".to_string();
        assert_eq!(gcd_of_strings(str1, str2), ret);
    }
}

fn main() {
    println!("Hello, world!");
}
