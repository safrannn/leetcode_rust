struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut helper_map: HashMap<i32, usize> = HashMap::new();
        for v in nums {
            *helper_map.entry(v).or_default() += 1;
        }
        let mut helper_heap: BinaryHeap<(Reverse<usize>, i32)> = BinaryHeap::new();
        for (i, v) in helper_map {
            helper_heap.push((Reverse(v), i));
            if helper_heap.len() > k as usize {
                helper_heap.pop();
            }
        }
        helper_heap.into_iter().map(|v| v.1).rev().collect()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
        vec![1, 2]
    );
    assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
}
