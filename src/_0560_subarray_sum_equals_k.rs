struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum: i32 = 0;
        let mut result: i32 = 0;
        let mut helper: HashMap<i32, i32> = HashMap::new();
        helper.insert(0, 1);

        for v in nums {
            sum += v;
            result += helper.get(&(sum - k)).unwrap_or(&0);
            *helper.entry(sum).or_insert(0) += 1;
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
}
