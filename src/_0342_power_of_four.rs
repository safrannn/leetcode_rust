struct Solution;

impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        return num > 0 && (num & (num - 1)) == 0 && (num & 0b1010101010101010101010101010101 != 0);
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_power_of_four(16), true);
    assert_eq!(Solution::is_power_of_four(5), false);
}
