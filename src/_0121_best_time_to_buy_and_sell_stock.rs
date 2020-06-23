struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_value: i32 = std::i32::MAX;
        let mut max_profit: i32 = 0;

        for v in prices.into_iter() {
            if v < min_value {
                min_value = v - 0;
            } else if v - min_value > max_profit {
                max_profit = v - min_value;
            }
        }

        max_profit
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
}
