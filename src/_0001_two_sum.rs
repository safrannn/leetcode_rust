struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut number_map = HashMap::with_capacity(nums.len());
        for (idx, num) in nums.into_iter().enumerate() {
            if number_map.contains_key(&num) {
                return vec![number_map[&num] as i32, idx as i32];
            } else {
                number_map.insert(target - num, idx);
            }
        }
        vec![]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}
