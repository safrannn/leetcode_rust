struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut current: i32 = nums[0];
        let mut occur: i32 = 0;

        for i in 0..nums.len() {
            if nums[i] == current {
                occur += 1;
            } else {
                occur -= 1;

                if occur == 0 {
                    current = nums[i];
                    occur = 1;
                }
            }
        }
        current
    }
}

#[test]
fn test() {
    assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
}
