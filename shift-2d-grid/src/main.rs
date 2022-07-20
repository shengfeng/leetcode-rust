pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let mut ret = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            let index = (i * n + j + k as usize) % (m * n);
            ret[index / n][index % n] = grid[i][j];
        }
    }
    ret
}


#[cfg(test)]
mod tests {
    use crate::shift_grid;

    #[test]
    fn test01() {
        let grid = vec![
            vec![1,2,3], vec![4,5,6], vec![7,8,9]
        ];
        let target = vec![
            vec![9,1,2], vec![3,4,5], vec![6,7,8]
        ];
        assert_eq!(shift_grid(grid, 1), target);
    }


    #[test]
    fn test02() {
        let grid = vec![
            vec![3,8,1,9], vec![19,7,2,5], 
            vec![4,6,11,10], vec![12,0,21,13]
        ];
        let target = vec![
            vec![12,0,21,13], vec![3,8,1,9],
            vec![19,7,2,5], vec![4,6,11,10]
        ];
        assert_eq!(shift_grid(grid, 4), target);
    }
}

fn main() {
    println!("Hello, world!");
}
