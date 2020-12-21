use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut sum = 0;
        let mut set: HashSet<i32> = HashSet::new();
        let mut left: usize = 0;
        let mut right: usize = 0;

        while right < nums.len() {
            let cur = nums[right];
            if set.contains(&cur) {
                while nums[left] != cur {
                    set.remove(&nums[left]);
                    sum -= nums[left];
                    left += 1;
                }
                left += 1;
            } else {
                set.insert(cur);
                sum += cur;
            }
            result = result.max(sum);
            right += 1;
        }
        result as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::maximum_unique_subarray(vec![4, 2, 4, 5, 6]), 17);
    assert_eq!(
        Solution::maximum_unique_subarray(vec![5, 2, 1, 2, 5, 2, 1, 2, 5]),
        8
    );
}
