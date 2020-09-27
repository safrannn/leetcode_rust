struct Solution;
impl Solution {
    pub fn min_operations_max_profit(
        customers: Vec<i32>,
        boarding_cost: i32,
        running_cost: i32,
    ) -> i32 {
        let mut wait: i32 = 0;
        let mut round: i32 = 0;
        let mut profit: i32 = 0;
        let mut profit_max: i32 = std::i32::MIN;
        let mut profit_max_round: i32 = 0;
        let mut customer_index: usize = 0;
        while customer_index < customers.len() || wait > 0 {
            if customer_index < customers.len() {
                wait += customers[customer_index];
                customer_index += 1;
            }
            profit += boarding_cost * wait.min(4) - running_cost;
            wait -= wait.min(4);
            round += 1;

            if profit > profit_max {
                profit_max = profit;
                profit_max_round = round;
            }
        }
        if profit_max < 0 {
            -1
        } else {
            profit_max_round
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_operations_max_profit(vec![8, 3], 5, 6), 3);
    assert_eq!(Solution::min_operations_max_profit(vec![10, 9, 6], 6, 4), 7);
    assert_eq!(
        Solution::min_operations_max_profit(vec![3, 4, 0, 5, 1], 1, 92),
        -1
    );
    assert_eq!(
        Solution::min_operations_max_profit(vec![10, 10, 6, 4, 7], 3, 8),
        9
    );
}
