struct Solution;

impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut dp = vec![];
        let mut left_count = vec![0; n];
        for (i, num) in nums.iter().enumerate() {
            left_count[i] = match dp.binary_search(&num) {
                Ok(k) => k + 1,
                Err(k) => {
                    if k == dp.len() {
                        dp.push(num);
                    } else {
                        dp[k] = num;
                    }
                    k + 1
                }
            };
        }

        let mut dp = vec![];
        let mut right_count = vec![0; n];
        for (i, num) in nums.iter().enumerate().rev() {
            right_count[i] = match dp.binary_search(&num) {
                Ok(k) => k + 1,
                Err(k) => {
                    if k == dp.len() {
                        dp.push(num);
                    } else {
                        dp[k] = num;
                    }
                    k + 1
                }
            };
        }

        let mut result = n;
        for i in 0..n {
            if left_count[i] != 1 && right_count[i] != 1 {
                result = result.min(n - (left_count[i] + right_count[i] - 1));
            }
        }
        result as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimum_mountain_removals(vec![1, 3, 1]), 0);
    assert_eq!(
        Solution::minimum_mountain_removals(vec![2, 1, 1, 5, 6, 2, 3, 1]),
        3
    );
    assert_eq!(
        Solution::minimum_mountain_removals(vec![4, 3, 2, 1, 1, 2, 3, 1]),
        4
    );
    assert_eq!(
        Solution::minimum_mountain_removals(vec![1, 2, 3, 4, 4, 3, 2, 1]),
        1
    );
    assert_eq!(
        Solution::minimum_mountain_removals(vec![4, 3, 2, 1, 1, 2, 3, 1]),
        4
    );
}
