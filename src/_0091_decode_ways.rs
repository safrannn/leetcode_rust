struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let n = s.len();
        let s_bytes = s.as_bytes();
        let mut dp: Vec<i32> = vec![0; n + 1];
        dp[0] = 1;

        for (i, b) in s_bytes.iter().enumerate() {
            if b - '0' as u8 > 0 {
                dp[i + 1] += dp[i];
            }
            if i == 0 {
                continue;
            }
            if (s_bytes[i - 1] - '0' as u8) * 10 + b - '0' as u8 >= 10
                && (s_bytes[i - 1] - '0' as u8) * 10 + b - '0' as u8 <= 26
            {
                dp[i + 1] += dp[i - 1];
            }
        }
        dp[n]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_decodings("12".to_string()), 2);
    assert_eq!(Solution::num_decodings("226".to_string()), 3);
    assert_eq!(Solution::num_decodings("0".to_string()), 0);
    assert_eq!(Solution::num_decodings("1".to_string()), 1);
}
