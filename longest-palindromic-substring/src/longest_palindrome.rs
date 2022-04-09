pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) {
        let ss = s.as_bytes();
        let L = s.len();
        let M = L * 2 - 1;
        let mut l = 0;
        let mut r = 0;
        let mut ans = &ss[0..1];

        println!("{:?}", ans);
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;
}
