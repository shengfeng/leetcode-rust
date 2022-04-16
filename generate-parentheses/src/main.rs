pub fn valid(s: &Vec<u8>) -> bool {
    let mut balance = 0;
    for &c in s {
        if c == b'(' {
            balance += 1;
        } else if c == b')' {
            balance -= 1;
        }
        if balance < 0 {
            return false;
        }
    }
    balance == 0
}

pub fn backtrack(max: usize, buf: &mut Vec<u8>, result: &mut Vec<String>) {
    println!("{:?}", buf);
    if buf.len() == max {
        // println!("{:?}", buf);
        if valid(&buf) {
            result.push(String::from_utf8(buf.to_vec()).unwrap());
        }
        return;
    }
    buf.push(b'(');
    backtrack(max, buf, result);
    buf.pop();

    buf.push(b')');
    backtrack(max, buf, result);
    buf.pop();
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut result = vec![];
    let max = (n * 2) as usize;
    backtrack(max, &mut vec![], &mut result);
    result
}

mod test {
    use crate::generate_parenthesis;

    #[test]
    fn test_01() {
        let n = 3;
        let target = vec![
            "((()))","(()())","(())()","()(())","()()()"];
        assert_eq!(generate_parenthesis(n), target);
    }

    #[test]
    fn test_02() {
        let n = 1;
        let target = vec!["()"];
        assert_eq!(generate_parenthesis(n), target);
    }
}


fn main() {
    let n = 3;
    let ret = generate_parenthesis(n);
    println!("{:?}", ret);
}
