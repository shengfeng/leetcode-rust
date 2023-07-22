pub fn max_freq(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32 {
    use std::collections::HashMap;
    use std::collections::HashSet;

    let n = s.len();
    let mut occ: HashMap<&str, i32> = HashMap::new();
    let mut ret = 0;
    let letters = s.bytes().collect::<Vec<_>>();
    for i in 0..n {
        let mut exist = HashSet::new();
        for j in i..n.min(i + max_size as usize) {
            exist.insert(letters[j]);
            if exist.len() > max_letters as usize {
                break;
            }
            let cur = &s[i..j+1];
            if j - i + 1 >= min_size as usize {
                let stat = occ.entry(&cur).or_insert(0);
                *stat += 1;
                ret = std::cmp::max(ret, *occ.get(&cur).unwrap());
            }
        }
    }
    ret
}


#[cfg(test)]
mod tests {
    use crate::max_freq;

    #[test]
    fn test_01() {
        let s = String::from("aababcaab");
        assert_eq!(max_freq(s, 2, 3, 4), 2);
    }


    #[test]
    fn test_02() {
        let s = String::from("aaaa");
        assert_eq!(max_freq(s, 1, 3, 3), 2);
    }


    #[test]
    fn test_03() {
        let s = String::from("aabcabcab");
        assert_eq!(max_freq(s, 2, 2, 3), 3);
    }


    #[test]
    fn test_04() {
        let s = String::from("abcde");
        assert_eq!(max_freq(s, 2, 3, 3), 0);
    }
}

fn main() {
    println!("Hello, world!");
}
