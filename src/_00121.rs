use crate::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut max_profit = 0;

        for i in 0..prices.len() {
            if prices[i] < min_price {
                min_price = prices[i];
            } else if prices[i] - min_price > max_profit {
                max_profit = prices[i] - min_price;
            }
        }

        max_profit
    }
}
