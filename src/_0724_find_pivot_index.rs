struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum_all: i32 = nums.iter().sum();
        let mut sum: i32 = 0;
        for (i, v) in nums.iter().enumerate() {
            if sum == sum_all - v - sum {
                return i as i32;
            }
            sum += v;
        }
        -1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
}
