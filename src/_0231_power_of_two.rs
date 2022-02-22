struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_power_of_two(1), true);
    assert_eq!(Solution::is_power_of_two(16), true);
    assert_eq!(Solution::is_power_of_two(3), false);
}
