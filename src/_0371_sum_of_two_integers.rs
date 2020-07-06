struct Solution;

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Solution::get_sum(a ^ b, (a & b) << 1)
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::get_sum(1, 2), 3);
    assert_eq!(Solution::get_sum(-2, 3), 1);
}
