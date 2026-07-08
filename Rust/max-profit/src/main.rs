#![allow(unused)]

struct Solution;

fn brutforce(prices: &[i32]) -> i32 {
    let mut max_profit = 0;
    for i in 0..prices.len() {
        for price_j in &prices[i..] {
            let profit = price_j - prices[i];
            max_profit = max_profit.max(profit);
        }
    }

    max_profit
}

fn the_solution(prices: &[i32]) -> i32 {
    let mut max_profit = 0;
    let Some(mut min_price) = prices.first() else {
        return max_profit;
    };

    for price in &prices[1..] {
        min_price = min_price.min(price);
        let profit = price - min_price;
        max_profit = max_profit.max(profit);
    }

    max_profit
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        the_solution(&prices)
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn profit() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let expected = 5;

        assert_eq!(Solution::max_profit(prices), expected);
    }
}
