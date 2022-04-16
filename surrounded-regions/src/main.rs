pub fn dfs(board: &mut Vec<Vec<char>>, i: i32, j: i32) {
    if i < 0 || i >= board.len() as i32 || j < 0 || j >= board[0].len() as i32 || board[i as usize][j as usize] == '#' || board[i as usize][j as usize] == 'X'  {
        return;
    }
    board[i as usize][j as usize] = '#';
    dfs(board, i - 1, j);
    dfs(board, i + 1, j);
    dfs(board, i, j - 1);
    dfs(board, i, j + 1);
}

pub fn solve(board: &mut Vec<Vec<char>>) {
    let row = board.len();
    let col = board[0].len();
    for i in 0..row {
        for j in 0..col {
            let is_edge = i == 0 || i == row - 1 || j == 0 || j == col - 1;
            if is_edge &&  board[i][j] == 'O' {
                dfs(board, i as i32, j as i32);
            }
        }
    }
    println!("{:?}", board);
    for i in 0..row {
        for j in 0..col {
            if board[i][j] == 'O' {
                board[i][j] = 'X';
            }
            if board[i][j] == '#' {
                board[i][j] = 'O';
            }
        }
    }
}


mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let mut board = vec![
            vec!['X','X','X','X'],
            vec!['X','O','O','X'],
            vec!['X','X','O','X'],
            vec!['X','O','X','X']
        ];
        let board_ret = vec![
            vec!['X','X','X','X'],
            vec!['X','X','X','X'],
            vec!['X','X','X','X'],
            vec!['X','O','X','X']
        ];
        solve(&mut board);
        assert_eq!(board, board_ret);
    }

    #[test]
    fn test_02() {
        let mut board = vec![vec!['X']];
        solve(&mut board);
        assert_eq!(board, vec![vec!['X']]);
    }
}


fn main() {
    let mut board = vec![
        vec!['X','X','X','X'],
        vec!['X','O','O','X'],
        vec!['X','X','O','X'],
        vec!['X','O','X','X']
    ];
    solve(&mut board);
}
