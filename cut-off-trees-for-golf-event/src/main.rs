use std::cmp::Reverse;

pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
    let mut trees = vec![];
    let row = forest.len();
    let col = forest[0].len();
    for i in 0..row {
        for j in 0..col {
            if forest[i][j] > 1 {
                trees.push((i, j));
            }
        }
    }

    trees.sort_by(|a, b| forest[a.0][a.1].cmp(&forest[b.0][b.1]));
    let mut cx = 0;
    let mut cy = 0;
    let mut ret = 0;
    for i in 0..trees.len() {
        println!("{:?}", forest[trees[i].0][trees[i].1]);
        let steps = astar(&forest, cx, cy, trees[i].0 as i32, trees[i].1 as i32);
        if steps == -1 {
            return -1;
        }
        ret += steps;
        cx = trees[i].0 as i32;
        cy = trees[i].1 as i32;
    }
    ret
}


pub fn astar(forest: &Vec<Vec<i32>>, sx: i32, sy: i32, tx: i32, ty: i32) -> i32 {
    use std::collections::BinaryHeap;

    let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    if sx == tx && sy == ty {
        return 0;
    }
    let row = forest.len();
    let col = forest[0].len();
    let mut queue =  BinaryHeap::new();
    let mut costed = vec![vec![i32::MAX; col]; row];
    costed[sx as usize][sy as usize] = (sx - tx).abs() + (sy - ty).abs();
    queue.push((Reverse(costed[sx as usize][sy as usize]), Reverse(0), Reverse(sx * col as i32 + sy)));
    while !queue.is_empty() {
        let (Reverse(cost), Reverse(dist), Reverse(loc)) = queue.pop().unwrap();
        let cx = loc / col as i32;
        let cy = loc % col as i32;
        if cx == tx && cy == ty {
            return dist;
        }
        for j in 0..4 {
            let nx = cx + dirs[j].0;
            let ny = cy + dirs[j].1;
            if nx >= 0 && nx < row as i32 && ny >= 0 && ny < col as i32 && forest[nx as usize][ny as usize] > 0 {
                let ncost = dist + 1 + (nx - tx).abs() + (ny - ty).abs();
                if ncost < costed[nx as usize][ny as usize] {
                    queue.push((Reverse(ncost), Reverse(dist+1), Reverse(nx * col as i32 + ny)));
                    costed[nx as usize][ny as usize] = ncost;
                }
            }
        }
    }
    return -1;
}


pub fn dijkstra(forest: &Vec<Vec<i32>>, sx: i32, sy: i32, tx: i32, ty: i32) -> i32 {
    use std::collections::BinaryHeap;

    let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    if sx == tx && sy == ty {
        return 0;
    }
    let row = forest.len();
    let col = forest[0].len();
    let mut queue = BinaryHeap::new();
    let mut visited = vec![vec![false; col]; row];
    queue.push((Reverse(0), Reverse(sx * col as i32 + sy)));
    visited[sx as usize][sy as usize] = true;
    while !queue.is_empty() {
        let (Reverse(dist), Reverse(loc)) = queue.pop().unwrap();
        for j in 0..4 {
            let nx = loc / col as i32 + dirs[j].0;
            let ny = loc % col as i32 + dirs[j].1;
            if nx >= 0 && nx < row as i32 && ny >= 0 && ny < col as i32 {
                if !visited[nx as usize][ny as usize] && forest[nx as usize][ny as usize] > 0 {
                    if nx == tx && ny == ty {
                        return dist + 1;
                    }
                    queue.push((Reverse(dist + 1), Reverse(nx * col as i32 + ny)));
                    visited[nx as usize][ny as usize] = true;
                }
            }
        }
    }
    return -1;
}

pub fn bfs(forest: &Vec<Vec<i32>>, sx: i32, sy: i32, tx: i32, ty: i32) -> i32 {
    use std::collections::VecDeque;
    let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    if sx == tx && sy == ty {
        return 0;
    }
    let row = forest.len();
    let col = forest[0].len();
    let mut step = 0;
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; col]; row];
    queue.push_back((sx, sy));
    visited[sx as usize][sy as usize] = true;

    while !queue.is_empty() {
        step += 1;
        let sz = queue.len();
        for i in 0..sz {
            let (cx, cy) = queue.pop_front().unwrap();
            for j in 0..4 {
                let nx = cx + dirs[j].0;
                let ny = cy + dirs[j].1;
                if nx >= 0 && nx < row as i32 && ny >= 0 && ny < col as i32 {
                    if !visited[nx as usize][ny as usize] && forest[nx as usize][ny as usize] > 0 {
                        if nx == tx && ny == ty {
                            return step;
                        }
                        queue.push_back((nx, ny));
                        visited[nx as usize][ny as usize] = true;
                    }
                }
            }
        }
    }
    return -1;
}

#[cfg(test)]
mod tests {
    use crate::cut_off_tree;

    #[test]
    fn test_01() {
        let forest = vec![vec![1,2,3], vec![0,0,4], vec![7,6,5]];
        assert_eq!(cut_off_tree(forest),  6);
    }

    #[test]
    fn test_02() {
        let forest = vec![vec![1,2,3], vec![0,0,0], vec![7,6,5]];
        assert_eq!(cut_off_tree(forest),  -1);
    }

    
    #[test]
    fn test_03() {
        let forest = vec![vec![2,3,4], vec![0,0,5], vec![8,7,6]];
        assert_eq!(cut_off_tree(forest),  6);
    }

    #[test]
    fn test_04() {
        let forest = vec![
            vec![54581641,64080174,24346381,69107959],
            vec![86374198,61363882,68783324,79706116],
            vec![668150,92178815,89819108,94701471],
            vec![83920491,22724204,46281641,47531096],
            vec![89078499,18904913,25462145,60813308]
        ];
        assert_eq!(cut_off_tree(forest), 57);
    }
}

fn main() {
    println!("Hello, world!");
}
