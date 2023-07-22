pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    if obstacle_grid.len() == 0 {
        return 0;
    }
    let m = obstacle_grid.len();
    let n = obstacle_grid[0].len();

    if obstacle_grid[0][0] == 1 || obstacle_grid[m - 1][n - 1] == 1 {
        return 0;
    }
    let mut dp = vec![vec![0; n as usize]; m as usize];
    for i in 0..m {
        if obstacle_grid[i][0] == 0 {
            dp[i][0] = 1;
        } else {
            break;
        }
    }
    for j in 0..n {
        if obstacle_grid[0][j] == 0 {
            dp[0][j] = 1;
        } else {
            break;
        }
    }
    for i in 1..m {
        for j in 1..n {
            if obstacle_grid[i][j] == 1 {
                continue;
            }
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
        }
    }
    dp[m as usize - 1][n as usize - 1]
}

#[cfg(test)]
mod tests {
    use crate::unique_paths_with_obstacles;

    #[test]
    fn test_01() {
        let obstacle_grid = vec![
            vec![0, 0, 0],
            vec![0 ,1, 0],
            vec![0, 0, 0]
        ];
        assert_eq!(unique_paths_with_obstacles(obstacle_grid), 2);
    }

    #[test]
    fn test_02() {
        let obstacle_grid = vec![
            vec![0, 1],
            vec![0, 0]
        ];
        assert_eq!(unique_paths_with_obstacles(obstacle_grid), 1);
    }

    #[test]
    fn test_03() {
        let obstacle_grid = vec![
            vec![1, 0],
        ];
        assert_eq!(unique_paths_with_obstacles(obstacle_grid), 0);
    }
}

fn main() {
    println!("Hello, world!");
}
