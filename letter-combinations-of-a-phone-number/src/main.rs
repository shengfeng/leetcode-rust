use std::collections::HashMap;

#[macro_export]
macro_rules! hashmap {
    ($($key: expr =>  $val: expr),*) => {
        {
            let mut map = HashMap::new();
            $(map.insert($key, $val); )*
            map
        }
    };
}


pub fn backtrack<'a>(
    s: &str, 
    result: &mut Vec<String>, 
    buf: &mut Vec<&'a str>, 
    map:  &'a HashMap<u8, &str>) {
    let current = match s.as_bytes().first() {
        Some(it) => it,
        _ => {
            if !buf.is_empty() {
                result.push(buf.iter().map(|&s| s).collect());
            }
            return;
        }
    };
    let mapping = map[current];
    for i in 0..mapping.len() {
        buf.push(&mapping[i..=i]);
        backtrack(&s[1..], result, buf, map);
        buf.pop();
    }
}

pub fn letter_combinations(digits: String) -> Vec<String> {
    let letters = hashmap! {
        b'2' => "abc",
        b'3' => "def",
        b'4' => "ghi",
        b'5' => "jkl",
        b'6' => "mno",
        b'7' => "pqrs",
        b'8' => "tuv",
        b'9' => "wxyz"
    };
    println!("{:?}", letters);
    let mut result = vec![];
    backtrack(&digits, &mut result, &mut vec![], &letters);
    result
}

mod test{
    use crate::letter_combinations;

    #[test]
    fn test_01() {
        let digits = "23".to_string();
        let ret = vec!["ad","ae","af","bd","be","bf","cd","ce","cf"];
        assert_eq!(letter_combinations(digits), ret);
    }

    #[test]
    fn test_02() {
        let digits = "".to_string();
        let ret: Vec<String> = vec![];
        assert_eq!(letter_combinations(digits), ret);
    }

    #[test]
    fn test_03() {
        let digits = "2".to_string();
        let ret = vec!["a", "b", "c"];
        assert_eq!(letter_combinations(digits), ret);
    }
}

fn main() {
    let digits = "23".to_string();
    let ret = letter_combinations(digits);
    println!("{:?}", ret);
}
