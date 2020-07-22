struct Solution;

impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.chunks(2).fold(0, |sum, pair| sum + pair[0])
    }
}
#[test]
fn test() {
    assert_eq!(Solution::array_pair_sum(vec![1, 4, 3, 2]), 4);
}
