pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.is_empty() {
        return 0;
    }
    let mut min_price = prices[0];
    let mut ret = 0;
    prices.iter().for_each(|&p| {
        if p < min_price {
            min_price = p;
        }
        ret = ret.max(p - min_price);
    });
    ret
}

#[cfg(test)]
mod tests {
    use crate::max_profit;

    #[test]
    fn test_01() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit(prices), 5);
    }

    #[test]
    fn test_02() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit(prices), 0);
    }
}

fn main() {
    println!("Hello, world!");
}
