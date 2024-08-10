fn main() {
    println!("Hello, world!");
}


pub fn compress(chars: &mut Vec<char>) -> i32 {
    let n = chars.len();
    let mut idx = 0;
    let mut count = 1;

    for i in 1..n {
        if chars[i - 1] == chars[i] {
            count += 1;
        } else {
            chars[idx] = chars[i - 1];
            idx += 1;
            if count > 1 {
                for c in count.to_string().chars() {
                    chars[idx] = c;
                    idx += 1;
                }
            }
            count = 1;
        }
    }

    chars[idx] = chars[n - 1];
    idx += 1;
    if count > 1 {
        for c in count.to_string().chars() {
            chars[idx] = c;
            idx += 1;
        }
    }

    idx as i32
}

#[cfg(test)]
mod tests {
    use crate::compress;

    #[test]
    fn test_01() {
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        assert_eq!(compress(&mut chars), 6);
    }

    #[test]
    fn test_02() {
        let mut chars = vec!['a'];
        assert_eq!(compress(&mut chars), 1);
    }

    #[test]
    fn test_03() {
        let mut chars = vec!['a','b','b','b','b','b','b','b','b','b','b','b','b'];
        assert_eq!(compress(&mut chars), 4);
    }
}