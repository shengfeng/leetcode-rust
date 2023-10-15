use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
}

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let num_courses = num_courses as usize;
    let mut graph = vec![vec![]; num_courses];
    let mut in_degrees = vec![0; num_courses];

    for v in prerequisites {
        let (i, j) = (v[0] as usize, v[1] as usize);
        graph[j].push(i);
        in_degrees[i] += 1;
    }

    let mut que = (0..num_courses).filter(|&i| in_degrees[i]== 0).collect::<VecDeque<_>>();
    let mut count = que.len();
    
    while let Some(cur_course) = que.pop_front() {
        for &next_course in graph[cur_course].iter() {
            in_degrees[next_course] -= 1;
            if in_degrees[next_course] == 0 {
                que.push_back(next_course);
                count += 1;
                if count == num_courses {
                    return true;
                }
            }
        }
    }
    count == num_courses
}

#[cfg(test)]
mod tests {
    use crate::can_finish;

    #[test]
    fn test_01() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0]];

        assert!(can_finish(num_courses, prerequisites));
    }

    #[test]
    fn test_02() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0], vec![0, 1]];

        assert_eq!(can_finish(num_courses, prerequisites), false);
    }
}