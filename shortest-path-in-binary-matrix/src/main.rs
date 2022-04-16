use std::collections::VecDeque;

pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    if grid[0][0] == 1 || grid[n - 1][n - 1] == 1 {
        return -1;
    }
    let neighbors: Vec<(i32, i32)> = vec![
        (0, -1), (0, 1), (-1, 0), (1, 0), 
        (-1, -1), (-1, 1), (1, -1), (1, 1)];
    let mut q = VecDeque::new();
    let mut visited = vec![vec![false; n]; n];
    q.push_back((0, 0));
    visited[0][0] = true;
    let mut ret = 0;

    while !q.is_empty() {
        let len = q.len();
        ret += 1;
        println!("{:?}", q);
        for i in 0..len {
            let (i, j) = q.pop_front().unwrap();
            if i == n as i32 - 1 && j == n as i32 - 1 {
                return ret;
            }
            println!("i = {}, j = {}", i, j);
            for neighbor in &neighbors {
                let next_i = i + neighbor.0;
                let next_j = j + neighbor.1;
                println!("next_i = {}, next_j = {}", next_i, next_j);
                if next_i >= 0 && next_i < n as i32  && next_j >= 0 && next_j < n as i32 &&
                    grid[next_i as usize][next_j as usize] == 0 && 
                    !visited[next_i as usize][next_j as usize] {
                        visited[next_i as usize][next_j as usize] = true;
                        q.push_back((next_i, next_j));
                }
            }
        }
    }
    -1
}


#[test]
fn test_01() {
    let grid = vec![vec![0,1], vec![1,0]];
    assert_eq!(shortest_path_binary_matrix(grid), 2);
}

#[test]
fn test_02() {
    let grid = vec![vec![0,0,0], vec![1,1,0], vec![1,1,0]];
    assert_eq!(shortest_path_binary_matrix(grid), 4);
}

#[test]
fn test_03() {
    let grid = vec![vec![1,0,0], vec![1,1,0], vec![1,1,0]];
    assert_eq!(shortest_path_binary_matrix(grid), -1);
}

#[test]
fn test_04() {
    /*
     [0, 0, 0, 1]
     [0, 0, 1, 0]
     [0, 1, 0, 0]
     [1, 0, 0, 0]
    */
    let grid = vec![vec![0,0,0,1], vec![0,0,1,0], vec![0,1,0,0], vec![1,0,0,0]];
    assert_eq!(shortest_path_binary_matrix(grid), 4);
}

#[test]
fn test_05() {
    let grid = vec![vec![0]];
    assert_eq!(shortest_path_binary_matrix(grid), 1);
}

#[test]
fn test_06() {
    let grid = vec![
        vec![0,0,0,0,1],
        vec![1,0,0,0,0],
        vec![0,1,0,1,0],
        vec![0,0,0,1,1],
        vec![0,0,0,1,0]];
    assert_eq!(shortest_path_binary_matrix(grid), -1);
}

fn main() {
    let grid = vec![vec![0,0,0,1], vec![0,0,1,0], vec![0,1,0,0], vec![1,0,0,0]];
    let ret = shortest_path_binary_matrix(grid);
    println!("{:?}", ret);
}
