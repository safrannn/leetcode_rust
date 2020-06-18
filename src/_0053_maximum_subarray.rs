struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut current_max = nums[0];
        let mut overall_max = nums[0];

        for i in 1..nums.len() {
            current_max = nums[i].max(current_max + nums[i]);
            overall_max = overall_max.max(current_max);
        }
        overall_max
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
        6
    );
}
