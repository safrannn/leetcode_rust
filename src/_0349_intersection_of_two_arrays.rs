use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let nums1 = nums1.into_iter().collect::<HashSet<i32>>();
        let nums2 = nums2.into_iter().collect::<HashSet<i32>>();
        (&nums1 & &nums2).into_iter().collect()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]),
        vec![2]
    );
}
