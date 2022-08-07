pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
    let mut ans = vec![0; n as usize];
    let mut stack = Vec::with_capacity(logs.len() >> 1);
    for log in logs {
        let log: Vec<_> = log.split(":").collect();
        let idx: usize = log[0].parse().unwrap();
        let time: i32 = log[2].parse().unwrap();

        if log[1] == "start" {
            stack.push((idx, time, 0));
        } else {
            let (idx, start, other) = stack.pop().unwrap();
            ans[idx] += time - start - other + 1;
            if let Some(t) = stack.last_mut() {
                t.2 += time - start + 1;
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use crate::exclusive_time;

    #[test]
    fn test_01() {
        let n = 2;
        let logs = vec![
            String::from("0:start:0"),String::from("1:start:2"),
            String::from("1:end:5"),String::from("0:end:6")];

        assert_eq!(exclusive_time(n, logs), vec![3, 4]);
    }

    #[test]
    fn test_02() {
        let n = 1;
        let logs = vec![
            String::from("0:start:0"),String::from("0:start:2"),
            String::from("0:end:5"),String::from("0:start:6"),
            String::from("0:end:6"),String::from("0:end:7")];

        assert_eq!(exclusive_time(n, logs), vec![8]);
    }
}

fn main() {
    println!("Hello, world!");
}
