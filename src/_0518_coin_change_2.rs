struct Solution;

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = vec![0; amount as usize + 1];
        dp[0] = 1;

        for coin in coins {
            for i in coin..=amount {
                dp[i as usize] += dp[i as usize - coin as usize];
            }
        }

        dp[amount as usize]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::change(5, vec![1, 2, 5]), 4);
    assert_eq!(Solution::change(3, vec![2]), 0);
    assert_eq!(Solution::change(10, vec![10]), 1);
}
