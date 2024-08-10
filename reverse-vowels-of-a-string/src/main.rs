fn main() {
    println!("Hello, world!");
}

pub fn is_vowel(c: char) -> bool {
    matches!(c, 'a'|'e'|'i'|'o'|'u'|'A'|'E'|'I'|'O'|'U')
}

pub fn reverse_vowels(s: String) -> String {
    let mut chars = s.chars().collect::<Vec<char>>();
    let (mut i, mut j) = (0, chars.len() - 1);
    while i < j {
        if !is_vowel(chars[i]) {
            i += 1;
            continue;
        }
        if !is_vowel(chars[j]) {
            j -= 1;
            continue;
        }
        chars.swap(i, j);
        i += 1;
        j -= 1;
    }
    chars.iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::reverse_vowels;

    #[test]
    fn test_01() {
        let s = String::from("hello");
        assert_eq!(reverse_vowels(s), String::from("holle"));
    }

    #[test]
    fn test_02() {
        let s = String::from("leetcode");
        assert_eq!(reverse_vowels(s), String::from("leotcede"));
    }
}
