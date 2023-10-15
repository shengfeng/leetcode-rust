use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
}

pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let num_courses = num_courses as usize;
    let mut graph = vec![vec![]; num_courses];
    let mut in_degrees = vec![0; num_courses];

    for v in prerequisites {
        let (i, j) = (v[0] as usize, v[1] as usize);
        graph[j].push(i);
        in_degrees[i] += 1;
    }

    let mut que = (0..num_courses).filter(|&i| in_degrees[i] == 0).collect::<VecDeque<_>>();
    let mut ret = vec![];

    while let Some(cur_course) = que.pop_front() {
        ret.push(cur_course as i32);
        for &next_course in graph[cur_course].iter() {
            in_degrees[next_course] -= 1;
            if in_degrees[next_course] == 0 {
                que.push_back(next_course);
            }
        }
    }
    if ret.len() == num_courses {
        return ret;
    } else {
        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use crate::find_order;

    #[test]
    fn test_01() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0]];
        let ret = vec![0, 1];

        assert_eq!(find_order(num_courses, prerequisites), ret);
    }

    #[test]
    fn test_02() {
        let num_courses = 4;
        let prerequisites = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
        let ret = vec![0, 1, 2, 3];

        assert_eq!(find_order(num_courses, prerequisites), ret);
    }

    #[test]
    fn test_03() {
        let num_courses = 1;
        let prerequisites = vec![];
        let ret = vec![0];

        assert_eq!(find_order(num_courses, prerequisites), ret);
    }
}