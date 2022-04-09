pub fn rebuild_string(s: &str) -> Vec<u8> {
    let mut ret = vec![];
    s.as_bytes().iter().for_each(|&c| {
        if c != b'#' {
            ret.push(c);
        } else {
            ret.pop();
        }
    });
    ret
}


pub fn backspace_compare(s: String, t: String) -> bool {
    let rebuild_s = rebuild_string(&s[..]);
    let rebuild_t = rebuild_string(&t[..]);
    rebuild_s == rebuild_t
}

#[test]
fn test_01() {
    let s = "ab#c".to_string();
    let t = "ad#c".to_string();
    let ret = backspace_compare(s, t);
    assert_eq!(ret, true);
}

#[test]
fn test02() { 
    let s = "ab##".to_string();
    let t = "c#d#".to_string();
    let ret = backspace_compare(s, t);
    assert_eq!(ret, true);
}

#[test]
fn test03() { 
    let s = "a#c".to_string();
    let t = "b".to_string();
    let ret = backspace_compare(s, t);
    assert_eq!(ret, false);
}

fn main() {
    println!("Hello, world!");
}
