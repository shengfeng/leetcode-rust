pub fn dfs(grid: &mut Vec<Vec<char>>, i: i32, j: i32) {
    if i < 0 || i >= grid.len() as i32 || j < 0 || j >= grid[0].len() as i32 {
        return;
    }
    if grid[i as usize][j as usize] == '0' {
        return;
    }
    grid[i as usize][j as usize] = '0';
    dfs(grid, i + 1, j);
    dfs(grid, i - 1, j);
    dfs(grid, i, j + 1);
    dfs(grid, i, j - 1);
}

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut ret = 0;
    let mut grid = grid;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '1' {
                ret += 1;
                dfs(&mut grid, i as i32, j as i32);
            }
        }
    }
    ret
}

#[test]
fn test_01() {
    let grid = vec![
        vec!['1','1','1','1','0'],
        vec!['1','1','0','1','0'],
        vec!['1','1','0','0','0'],
        vec!['0','0','0','0','0']];
    assert_eq!(num_islands(grid), 1);
}

#[test]
fn test_02() {
    let grid = vec![
        vec!['1','1','0','0','0'],
        vec!['1','1','0','0','0'],
        vec!['0','0','1','0','0'],
        vec!['0','0','0','1','1']];
    assert_eq!(num_islands(grid), 3);
}


fn main() {
    println!("Hello, world!");
}
