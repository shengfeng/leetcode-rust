pub fn calculate(s: String) -> i32 {
    enum Operator {
        Add,
        Sub,
        Mul,
        Div,
        Ext
    }
    let mut num_vec = vec![];
    let mut num = 0;
    let mut operator = Operator::Add;
    for ch in (s + ".").chars() {
        if ch == ' ' {
            continue;
        }
        if ('0'..='9').contains(&ch) {
            num = num * 10 + (ch as u8 - b'0') as i32;
        } else {
            match operator {
                Operator::Add => num_vec.push(num),
                Operator::Sub => num_vec.push(-num),
                Operator::Mul => *num_vec.last_mut().unwrap() *= num,
                Operator::Div => *num_vec.last_mut().unwrap() /= num,
                Operator::Ext => {}
            };

            // get the infix operator 
            operator = match ch {
                '+' => Operator::Add,
                '-' => Operator::Sub,
                '*' => Operator::Mul,
                '/' => Operator::Div,
                _ => Operator::Ext,
            };
            num = 0;
        }
    }
    num_vec.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::calculate;

    #[test]
    fn test_01() {
        let s = "3+2*2".to_string();
        assert_eq!(calculate(s), 7);
    }

    #[test]
    fn test_02() {
        let s = "3/2".to_string();
        assert_eq!(calculate(s), 1);
    }

    #[test]
    fn test_03() {
        let s = " 3+5 / 2 ".to_string();
        assert_eq!(calculate(s), 5);
    }
}

fn main() {
    println!("Hello, world!");
}
