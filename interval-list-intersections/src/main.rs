pub fn interval_intersection(first_list: Vec<Vec<i32>>, second_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::cmp;
    let mut ret = vec![];
    let mut i = 0;
    let mut j = 0;
    while i < first_list.len() && j < second_list.len() {
        let start = cmp::max(first_list[i][0], second_list[j][0]);
        let end = cmp::min(first_list[i][1], second_list[j][1]);
        if start <= end {
            ret.push(vec![start, end]);
        }
        if first_list[i][1] < second_list[j][1] {
            i += 1;
        } else {
            j += 1;
        }
    }
    ret
}

#[test]
fn test_01() {
    let first_list = vec![vec![0,2], vec![5,10], vec![13,23], vec![24,25]];
    let second_list = vec![vec![1,5], vec![8,12], vec![15,24], vec![25,26]];
    let ret = interval_intersection(first_list, second_list);
    assert_eq!(ret, vec![vec![1,2], vec![5,5], vec![8,10], vec![15,23], vec![24,24], vec![25,25]]);
}

fn main() {
    println!("Hello, world!");
}
