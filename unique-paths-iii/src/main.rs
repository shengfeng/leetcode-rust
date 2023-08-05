pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut left = 0;
    let mut x = 0;
    let mut y = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                left += 1;
            }

            if grid[i][j] == 1 {
                x = i as i32;
                y = j as i32;
                left += 1;
            }
        }
    }
    dfs(&mut grid, x, y, left)
}


pub fn dfs(mut grid: &mut Vec<Vec<i32>>, i: i32, j: i32, mut left: i32) -> i32 {
    if i < 0 || j < 0 || i >= grid.len() as i32 || j >= grid[0].len() as i32 || grid[i as usize][j as usize] == -1 {
        return 0;
    }

    if grid[i as usize][j as usize] == 2 {
        if left == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    grid[i as usize][j as usize] = -1;
    left -= 1;
    let res = dfs(grid, i - 1, j, left) + 
        dfs(grid, i + 1, j, left) + 
        dfs(grid, i, j - 1, left) + 
        dfs(grid, i, j + 1, left);
    
    // bracktrack
    grid[i as usize][j as usize] = 0;
    left += 1;
    res
}

#[cfg(test)]
mod tests {
    use crate::unique_paths_iii;


    #[test]
    fn test_01() {
        let grid = vec![
            vec![1, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 2, -1]
        ];
        assert_eq!(unique_paths_iii(grid), 2);
    }

    #[test]
    fn test_02() {
        let grid = vec![
            vec![1, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 2]
        ];
        assert_eq!(unique_paths_iii(grid), 4);
    }

    #[test]
    fn test_03() {
        let grid = vec![
            vec![0, 1],
            vec![2, 0],        
        ];
        assert_eq!(unique_paths_iii(grid), 0);
    }
}

fn main() {
    println!("Hello, world!");
}
