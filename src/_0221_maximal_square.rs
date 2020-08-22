struct Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }
        let n: usize = matrix.len();
        let m: usize = matrix[0].len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; m + 1]; n + 1];
        let mut result: i32 = 0;
        for i in 1..=n {
            for j in 1..=m {
                if matrix[i - 1][j - 1] == '1' {
                    dp[i][j] = dp[i - 1][j].min(dp[i][j - 1].min(dp[i - 1][j - 1])) + 1;
                    result = result.max(dp[i][j]);
                }
            }
        }
        result * result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::maximal_square(vec_vec_char![]), 0);
    assert_eq!(Solution::maximal_square(vec_vec_char![['1', '1']]), 1);
}
