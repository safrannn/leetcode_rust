struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        if k < 0 {
            return 0;
        }
        if nums.len() < 2 {
            return 0;
        }
        let mut h_map: HashMap<i32, bool> = HashMap::new();

        for v in nums {
            if let Some(x) = h_map.get_mut(&v) {
                *x = true;
            } else {
                h_map.insert(v, false);
            }
        }

        let mut result: i32 = 0;
        if k == 0 {
            for (_, value) in h_map.into_iter() {
                if value {
                    result += 1;
                }
            }
        } else {
            for (key, _) in h_map.iter() {
                let target: i32 = k + key;
                if h_map.contains_key(&target) {
                    result += 1;
                }
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_pairs(vec![3, 1, 4, 1, 5], 2), 2);
    assert_eq!(Solution::find_pairs(vec![1, 2, 3, 4, 5], 1), 4);
    assert_eq!(Solution::find_pairs(vec![1, 3, 1, 5, 4], 0), 1);
}
