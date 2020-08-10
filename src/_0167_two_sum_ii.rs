struct Solution;

use std::cmp::Ordering::*;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left: usize = 0;
        let mut right: usize = numbers.len() - 1;
        while left < right {
            match (numbers[left] + numbers[right]).cmp(&target) {
                Equal => {
                    return vec![left as i32 + 1, right as i32 + 1];
                }
                Greater => {
                    right -= 1;
                }
                Less => left += 1,
            }
        }
        return vec![-1, -1];
    }
}

#[test]
fn test() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
}
