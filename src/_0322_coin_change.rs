struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp: Vec<i32> = vec![amount + 1; amount as usize + 1];
        dp[0] = 0;

        for i in 1..=amount as usize {
            for &coin in coins.iter() {
                if coin <= i as i32 {
                    dp[i] = dp[i].min(dp[i - coin as usize] + 1);
                }
            }
        }

        if dp[amount as usize] == amount + 1 {
            return -1;
        } else {
            return dp[amount as usize];
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    assert_eq!(Solution::coin_change(vec![2], 3), -1);
}
