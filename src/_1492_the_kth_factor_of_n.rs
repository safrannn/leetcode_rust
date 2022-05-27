struct Solution;

impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        (1..=n)
            .into_iter()
            .filter(|&x| n % x == 0)
            .nth(k as usize - 1)
            .unwrap_or(-1)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::kth_factor(12, 3), 3);
    assert_eq!(Solution::kth_factor(7, 2), 7);
    assert_eq!(Solution::kth_factor(4, 4), -1);
}
