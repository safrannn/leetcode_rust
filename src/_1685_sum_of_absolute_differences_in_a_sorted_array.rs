struct Solution;
impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let sum: i32 = nums.iter().sum();
        let mut prefix_sum = 0;
        let mut result = vec![];

        for (i, v) in nums.iter().enumerate() {
            result.push((sum - prefix_sum) - v * (n as i32 - i as i32) + v * i as i32 - prefix_sum);
            prefix_sum += v;
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::get_sum_absolute_differences(vec![2, 3, 5]),
        vec![4, 3, 5]
    );
    assert_eq!(
        Solution::get_sum_absolute_differences(vec![1, 4, 6, 8, 10]),
        vec![24, 15, 13, 15, 21]
    );
}
