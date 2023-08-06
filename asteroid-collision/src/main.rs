pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut num_vec = vec![];
    for aster in asteroids {
        let mut alive = true;
        while alive && aster < 0 && !num_vec.is_empty() && *num_vec.last().unwrap() > 0 {
            let top_num = num_vec.last().unwrap();
            alive = *top_num < -aster;
            if *top_num <= -aster {
                num_vec.pop();
            }
        }
        if alive {
            num_vec.push(aster);
        }
    }
    num_vec
}

#[cfg(test)]
mod tests {
    use crate::asteroid_collision;

    #[test]
    fn test_01() {
        let asteroids = vec![5, 10, -5];
        let ret = vec![5, 10];
        assert_eq!(asteroid_collision(asteroids), ret);
    }

    #[test]
    fn test_02() {
        let asteroids = vec![8, -8];
        let ret = vec![];
        assert_eq!(asteroid_collision(asteroids), ret);
    }

    #[test]
    fn test_03() {
        let asteroids = vec![10, 2, -5];
        let ret = vec![10];
        assert_eq!(asteroid_collision(asteroids), ret);
    }
}

fn main() {
    println!("Hello, world!");
}
