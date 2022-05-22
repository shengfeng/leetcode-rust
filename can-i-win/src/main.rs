pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
    if (1 + max_choosable_integer) * max_choosable_integer / 2 < desired_total {
        return false;
    }
    // visited[state] == 0: not used, visited[state] == 1: true, visited[state] == 2: false
    let mut visited = vec![0; 1 << (max_choosable_integer+1)];
    dfs(0, 0, &mut visited, max_choosable_integer, desired_total)
}

pub fn dfs(state: i32, current_total: i32, visited: &mut Vec<i32>, max_choosable_integer: i32, desired_total: i32) -> bool {
    if visited[state as usize] == 1 {
        return true;
    }

    if visited[state as usize] == 2 {
        return false;
    }

    for i in 1..=max_choosable_integer {
        if state >> i & 1 == 1 {
            continue; 
        }
        if current_total + i >= desired_total {
            visited[state as usize] = 1;
            return true;
        }
        if !dfs(state | (1 << i) , current_total + i, visited, max_choosable_integer, desired_total) {
            visited[state as usize] = 1;
            return true;
        }
    }
    visited[state as usize] = 2;
    false
}

#[cfg(test)]
mod tests {
    use crate::can_i_win;

    #[test]
    fn test_01() {
        let max_choosable_integer = 10;
        let desired_total = 11;
        assert_eq!(can_i_win(max_choosable_integer, desired_total), false);
    }

    #[test]
    fn test_02() {
        let max_choosable_integer = 10;
        let desired_total = 0;
        assert_eq!(can_i_win(max_choosable_integer, desired_total), true);
    }

    #[test]
    fn test_03() {
        let max_choosable_integer = 10;
        let desired_total = 1;
        assert_eq!(can_i_win(max_choosable_integer, desired_total), true);
    }

    #[test]
    fn test_04() {
        let max_choosable_integer = 20;
        let desired_total = 210;
        assert_eq!(can_i_win(max_choosable_integer, desired_total), false);
    }
}

fn main() {
    println!("Hello, world!");
}
