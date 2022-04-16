pub fn dfs(graph: &Vec<Vec<i32>>, cur: usize, road:&mut Vec<i32>, roads: &mut Vec<Vec<i32>>) {
    println!("{:?}", road);
    if cur == graph.len() - 1 {
        roads.push(road.to_vec());
        return;
    }
    for &g in &graph[cur] {
        road.push(g);
        dfs(graph, g as usize, road, roads);
        road.pop();
    }
}

pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut roads = vec![];
    let mut road = vec![];
    road.push(0);
    dfs(&graph, 0,  &mut road, &mut roads);
    roads
}

mod tests {
    use crate::all_paths_source_target;

    #[test]
    fn test_01() {
        let graph = vec![vec![1,2], vec![3], vec![3], vec![]];
        assert_eq!(all_paths_source_target(graph), vec![vec![0,1,3], vec![0,2,3]]);
    }
}

fn main() {
    println!("Hello, world!");
}
