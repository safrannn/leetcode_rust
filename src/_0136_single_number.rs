struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for v in nums {
            result ^= v;
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
    assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
}
