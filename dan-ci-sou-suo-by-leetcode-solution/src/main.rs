pub fn check(board: &Vec<Vec<char>>, word: &Vec<char>, x: i32, y: i32, k: usize, visited: &mut  Vec<Vec<bool>>) -> bool {
    if board[x as usize][y as usize] != word[k] {
        return false;
    } else if k == word.len() - 1 {
        return true;
    }
    visited[x as usize][y as usize] = true;
    let neighbors: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut result = false;
    for i in 0..neighbors.len() {
        let new_x = neighbors[i].0 + x;
        let new_y = neighbors[i].1 + y;
        if new_x >= 0 && new_x < board.len() as i32 && new_y >= 0 && new_y < board[0].len() as i32 {
            if !visited[new_x as usize][new_y as usize] {
                let flag = check(board, word, new_x, new_y, k + 1, visited);
                if flag {
                    result = true;
                    break;
                }
            }
        }
    }
    visited[x as usize][y as usize] = false;
    result
}

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let row = board.len();
    let col = board[0].len();
    let mut visited = vec![vec![false; col]; row];
    let word = word.chars().collect::<Vec<char>>();
    for i in 0..row {
        for j in 0..col {
            let flag = check(&board, &word, i as i32, j as i32, 0, &mut visited);
            if flag {
                return true;
            }
        }
    }
    return false;
}

mod tests {
    use crate::exist;

    #[test]
    fn test_01() {
        let board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','C','S'],
            vec!['A','D','E','E']
        ];
        let word = String::from("ABCCED");
        assert_eq!(exist(board, word), true);
    }

    #[test]
    fn test_02() {
        let board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','C','S'],
            vec!['A','D','E','E']
        ];
        let word = String::from("SEE");
        assert_eq!(exist(board, word), true);
    }

    #[test]
    fn test_03() {
        let board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','C','S'],
            vec!['A','D','E','E']
        ];
        let word = String::from("ABCB");
        assert_eq!(exist(board, word), false);
    }
}


fn main() {
    println!("Hello, world!");
}
