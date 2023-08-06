/* hkip_hvc_dispatch */

pub fn can_place_flowers(flowerbed: Vec<i32>, mut n: i32) -> bool {
    let mut i = 0;
    let len = flowerbed.len();
    while i < len {
        if flowerbed[i] == 1 {
            i += 2;
        } else if i == len - 1 || flowerbed[i + 1] == 0 {
            n -= 1;
            i += 2;
        } else {
            i += 3;
        }
    }
    n <= 0
}

#[cfg(test)]
mod tests {
    use crate::can_place_flowers;

    #[test]
    fn test_01() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 1;
        assert_eq!(can_place_flowers(flowerbed, n), true);
    }

    #[test]
    fn test_02() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 2;
        assert_eq!(can_place_flowers(flowerbed, n), false);
    }

    #[test]
    fn test_03() {
        let flowerbed = vec![1, 0, 1, 0, 0];
        let n = 1;
        assert_eq!(can_place_flowers(flowerbed, n), true);
    }
}


fn main() {
    println!("Hello, world!");
}
