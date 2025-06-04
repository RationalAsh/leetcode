pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (_min_price, max_profit) = prices.iter().fold(
            (i32::MAX as i32, 0 as i32),
            |(_min_price, _max_profit), &p| {
                if p < _min_price {
                    (p, _max_profit)
                } else if (p - _min_price) > _max_profit {
                    (_min_price, p - _min_price)
                } else {
                    (_min_price, _max_profit)
                }
            },
        );

        max_profit
    }
}
