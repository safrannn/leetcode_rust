struct Solution;

impl Solution {
    pub fn min_cost(s: String, cost: Vec<i32>) -> i32 {
        let s = s.as_bytes().to_owned();
        let mut block_sum = 0;
        let mut block_max = 0;
        let mut result = 0;
        for i in 0..s.len() {
            if i > 0 && s[i] != s[i - 1] {
                result += block_sum - block_max;
                block_sum = 0;
                block_max = 0;
            }
            block_sum += cost[i];
            block_max = block_max.max(cost[i]);
        }
        result += block_sum - block_max;
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::min_cost("abaac".to_string(), vec![1, 2, 3, 4, 5]),
        3
    );
    assert_eq!(Solution::min_cost("abc".to_string(), vec![1, 2, 3]), 0);
    assert_eq!(
        Solution::min_cost("aabaa".to_string(), vec![1, 2, 3, 4, 1]),
        2
    );
}
