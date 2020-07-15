struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let sum1: i32 = (1 + nums.len() as i32) * (nums.len() as i32) / 2;
        let sum2: i32 = nums.iter().sum();
        sum1 - sum2
    }
}

#[test]
fn test() {
    assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
    assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
}
