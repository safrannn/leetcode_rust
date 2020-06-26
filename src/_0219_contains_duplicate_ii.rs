struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map: HashMap<i32, usize> = HashMap::new();
        let k = k as usize;
        for (i, v) in nums.into_iter().enumerate() {
            if let Some(j) = map.get(&v) {
                if i - j <= k {
                    return true;
                }
            }
            map.insert(v, i);
        }
        false
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3),
        true
    );
    assert_eq!(
        Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1),
        true
    );
    assert_eq!(
        Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2),
        false
    );
}
