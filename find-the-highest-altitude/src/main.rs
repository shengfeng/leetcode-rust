pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut v_ret = vec![0; gain.len() + 1];
    for i in 1..v_ret.len() {
        v_ret[i] = v_ret[i - 1] + gain[i - 1];
    }

    return *v_ret.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::largest_altitude;

    #[test]
    fn test_01() {
        let gain = vec![-5, 1, 5, 0, -7];
        assert_eq!(largest_altitude(gain), 1);
    }

    #[test]
    fn test_02() {
        let gain = vec![-4, -3, -2, -1, 4, 3, 2];
        assert_eq!(largest_altitude(gain), 0);
    }
}


fn main() {
    println!("Hello, world!");
}
