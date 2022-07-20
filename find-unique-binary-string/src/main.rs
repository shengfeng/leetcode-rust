pub fn find_different_binary_string(nums: Vec<String>) -> String {
    let n = nums.len();
    let mut ret = String::new();
    for i in 0..n {
        if nums[i].chars().collect::<Vec<char>>()[i] == '0' {
            ret.push('1');
        } else {
            ret.push('0');
        }
    }
    ret
}


#[cfg(test)]
mod tests {
    use crate::find_different_binary_string;

    #[test]
    fn test_01() {
        let nums = vec![String::from("01"), String::from("10")];
        let ret = String::from("11");
        assert_eq!(find_different_binary_string(nums), ret);
    }

    #[test]
    fn test_02() {
        let nums = vec![String::from("00"), String::from("01")];
        let ret = String::from("10");
        assert_eq!(find_different_binary_string(nums), ret);
    }

    #[test]
    fn test_03() {
        let nums = vec![
            String::from("111"), String::from("011"), String::from("001")];
        let ret = String::from("000");
        assert_eq!(find_different_binary_string(nums), ret);
    }
}

fn main() {
    println!("Hello, world!");
}
