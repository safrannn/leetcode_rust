use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut occur: HashMap<i32, i32> = HashMap::new();
        nums.into_iter()
            .for_each(|num| *occur.entry(num).or_default() += 1);
        let mut result = 0;
        occur.into_iter().for_each(|(k, v)| {
            if v == 1 {
                result += k;
            }
        });
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::sum_of_unique(vec![1, 2, 3, 2]), 4);
    assert_eq!(Solution::sum_of_unique(vec![1, 1, 1, 1, 1]), 0);
    assert_eq!(Solution::sum_of_unique(vec![1, 2, 3, 4, 5]), 15);
}
