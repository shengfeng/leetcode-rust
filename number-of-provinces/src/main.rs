pub fn dfs(is_connected: &Vec<Vec<i32>>, visited: &mut Vec<bool>, i: usize) {
    let provinces = is_connected.len();
    for j in 0..provinces {
        if is_connected[i][j] == 1 && !visited[j] {
            visited[j] = true;
            dfs(is_connected, visited, j);
        }
    }
}

pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let provinces = is_connected.len();
    let mut ret = 0;
    let mut visited = vec![false; provinces];
    for i in 0..provinces {
        if !visited[i] {
            dfs(&is_connected, &mut visited, i);
            ret += 1;
        }
    }
    ret
}


#[test]
fn test_01() {
    let is_connected = vec![vec![1,1,0], vec![1,1,0], vec![0,0,1]];
    assert_eq!(find_circle_num(is_connected), 2);
}

#[test]
fn test_02() {
    let is_connected = vec![vec![1,0,0], vec![0,1,0], vec![0,0,1]];
    assert_eq!(find_circle_num(is_connected), 3);
}


fn main() {
    println!("Hello, world!");
}
