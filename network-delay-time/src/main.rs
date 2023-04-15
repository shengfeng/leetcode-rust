pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let mut graph = vec![vec![i32::MAX; n as usize]; n as usize];
    times.iter().for_each(|t| {
        let x = t[0] - 1;
        let y = t[1] - 1;
        graph[x as usize][y as usize] = t[2];
    });

    let mut dist = vec![i32::MAX; n as usize];
    let mut visited = vec![false; n as usize];
    dist[k as usize -  1] = 0;

    for _i in 0..n {
        let mut min_dist = i32::MAX;
        let mut u: i32 = -1;
        for j in 0..n as usize {
            if !visited[j] && dist[j] < min_dist {
                u = j as i32;
                min_dist = dist[j];
            }
        }

        if u == -1 {
            return -1;
        }
        visited[u as usize] = true;
        for k in 0..n as usize {
            if !visited[k] && graph[u as usize][k] != i32::MAX {
                dist[k] = dist[k].min(dist[u as usize] + graph[u as usize][k]);
            }
        }
    }
    *dist.iter().max().unwrap()
}


#[cfg(test)]
mod tests {
    use crate::network_delay_time;

    #[test]
    fn test_01() {
        let times = vec![
            vec![2, 1, 1],
            vec![2, 3, 1],
            vec![3, 4, 1]
        ];

        assert_eq!(network_delay_time(times, 4, 2), 2);
    }

    #[test]
    fn test_02() {
        let times = vec![
            vec![1, 2, 1],
        ];

        assert_eq!(network_delay_time(times, 2, 1), 1);
    }

    #[test]
    fn test_03() {
        let times = vec![
            vec![1, 2, 1],
        ];

        assert_eq!(network_delay_time(times, 2, 2), -1);
    }
}


fn main() {
    println!("Hello, world!");
}
