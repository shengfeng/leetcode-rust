pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let p_mp = p.as_bytes().into_iter().fold(vec![0; 26], |mut mp, c| {
        mp[(c - b'a') as usize] += 1;
        mp
    });
    let s: Vec<usize> = s.as_bytes().into_iter().map(|c| (c - b'a') as usize).collect();
    let mut s_mp = vec![0; 26];
    let mut l = 0;
    let mut r = 0;
    let mut ret = vec![];
    
    while r < p.len().min(s.len()) - 1 {
        s_mp[s[r]] += 1;
        r += 1;
    }

    while r < s.len() {
        s_mp[s[r]] += 1;
        if p_mp == s_mp {
            ret.push(l as i32);
        }
        s_mp[s[l]] -= 1;
        l += 1;
        r += 1;
    }
    ret
}


#[test]
fn test_01() {
    let s = "cbaebabacd".to_string();
    let p = "abc".to_string();
    assert_eq!(find_anagrams(s, p), vec![0, 6]);
}

#[test]
fn test_02() {
    let s = "abab".to_string();
    let p = "ab".to_string();
    assert_eq!(find_anagrams(s, p), vec![0, 1, 2]);
}


fn main() {
    println!("Hello, world!");
}
