struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut h_map: HashMap<i32, i32> = HashMap::new();
        let mut h_stack: Vec<i32> = vec![];

        for v in nums2 {
            while let Some(last) = h_stack.pop() {
                if v >= last {
                    h_map.insert(last, v);
                } else {
                    h_stack.push(last);
                    break;
                }
            }
            h_stack.push(v);
        }
        nums1.iter().map(|v| *h_map.get(v).unwrap_or(&-1)).collect()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
        vec![-1, 3, -1]
    );
    assert_eq!(
        Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
        vec![3, -1]
    );
}
