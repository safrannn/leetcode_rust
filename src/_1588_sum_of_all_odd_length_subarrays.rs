struct Solution;

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        for i in 0..arr.len() as i32 {
            result += arr[i as usize] * (((i + 1) * (arr.len() as i32 - i) + 1) / 2);
        }
        result
    }
}
#[test]
fn test() {
    assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]), 58);
    assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 2]), 3);
    assert_eq!(Solution::sum_odd_length_subarrays(vec![10, 11, 12]), 66);
}
