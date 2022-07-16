// dp[i] = min(dp[i], dp[i - coins[j]]) + 1, j \in coins
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let max = amount + 1;
    let mut dp = vec![max; max as usize];
    dp[0] = 0;
    for i in 1..max as usize {
        for &coin in coins.iter() {
            if coin <= i as i32 {
                dp[i] = std::cmp::min(
                    dp[i], dp[i - coin as usize] + 1);
            }
        }
    }
    if dp[amount as usize] == max {
        -1
    } else {
        dp[amount as usize]
    }
}


mod tests {
    use crate::coin_change;


    #[test]
    fn test_01() {
        let coins = vec![1, 2, 5];
        let amount = 11;
        assert_eq!(coin_change(coins, amount), 3);
    }

    #[test]
    fn test_02() {
        let coins = vec![2];
        let amount = 3;
        assert_eq!(coin_change(coins, amount), -1);
    }

    #[test]
    fn test_03() {
        let coins = vec![1];
        let amount = 0;
        assert_eq!(coin_change(coins, amount), 0);
    }
}

fn main() {
    println!("Hello, world!");
}
