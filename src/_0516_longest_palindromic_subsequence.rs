struct Solution;

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; n + 1];

        for i in 0..n {
            dp[1][i] = 1;
        }

        for length in 2..=n {
            for start in 0..=n - length {
                dp[length][start] = if s[start] == s[start + length - 1] {
                    dp[length - 2][start + 1] + 2
                } else {
                    dp[length - 1][start].max(dp[length - 1][start + 1])
                }
            }
        }
        dp[n][0]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::longest_palindrome_subseq("bbbab".to_string()), 4);
    assert_eq!(Solution::longest_palindrome_subseq("cbbd".to_string()), 2);
}
