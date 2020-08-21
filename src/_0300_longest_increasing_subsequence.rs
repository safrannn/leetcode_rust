struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();
        if n == 0 {
            return 0;
        }
        let mut dp: Vec<i32> = vec![1; n];
        let mut max_val: i32 = 1;

        for i in 1..n {
            let mut max_i: i32 = 0;
            for j in 0..i {
                if nums[i] > nums[j] {
                    max_i = max_i.max(dp[j]);
                }
            }
            dp[i] = max_i + 1;
            max_val = max_val.max(dp[i]);
        }
        max_val
    }
}

#[test]
fn test() {
    assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
}
