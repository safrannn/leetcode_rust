use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut occurs: HashMap<i32, i32> = HashMap::new();
        let mut result = 0;
        for v1 in nums {
            let v2 = k - v1;
            if let Some(occur) = occurs.get_mut(&v2) {
                if *occur > 0 {
                    *occur -= 1;
                    result += 1;
                    continue;
                }
            }
            *occurs.entry(v1).or_default() += 1;
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_operations(vec![1, 2, 3, 4], 5), 2);
    assert_eq!(Solution::max_operations(vec![3, 1, 3, 4, 3], 6), 1);
}
