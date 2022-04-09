pub fn max_area(height: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = height.len() - 1;
    let mut ret = 0;
    while l < r {
        let area = height[l].min(height[r]) * (r - l) as i32;
        ret = ret.max(area);
        if height[l] <  height[r] {
            l += 1;
        } else {
            r -= 1;
        }
    }
    ret
}

#[test]
fn test_01() {
    let height = vec![1,8,6,2,5,4,8,3,7];
    let ret = max_area(height);
    assert_eq!(ret, 49);
}


#[test]
fn test_02() {
    let height = vec![1,1];
    let ret = max_area(height);
    assert_eq!(ret, 1);
}

fn main() {
    println!("Hello, world!");
}
