pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    if nums.len() < 3 {
        return ret;
    }
    let mut nums = nums;
    nums.sort();
    let n = nums.len();
    for first in 0..n {
        if nums[first] > 0 {
            return ret;
        }
        if first > 0 && nums[first] == nums[first - 1] {
            continue;
        }
        let mut third = n - 1;
        let target = -nums[first];
        for second in first+1..n {
            if second > first + 1 && nums[second] == nums[second - 1] {
                continue;
            }
            while second < third && nums[second] + nums[third] >  target {
                third -= 1;
            }
            if second == third {
                break;
            }
            if nums[second] + nums[third] == target {
                ret.push(vec![nums[first], nums[second], nums[third]]);
            }
        }
    }
    ret
}


#[test]
fn test_01() {
    let nums = vec![-1,0,1,2,-1,-4];
    let ret = vec![vec![-1,-1,2], vec![-1,0,1]];
    assert_eq!(ret, three_sum(nums));
}

#[test]
fn test_02() {
    let nums = vec![0];
    let ret: Vec<Vec<i32>> = vec![];
    assert_eq!(ret, three_sum(nums));
}

#[test]
fn test_03() {
    let nums = vec![];
    let ret: Vec<Vec<i32>> = vec![];
    assert_eq!(ret, three_sum(nums));
}

fn main() {
    println!("Hello, world!");
}
