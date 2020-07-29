struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut h_previous_max: i32 = 0;
        let mut h_current_max: i32 = 0;

        for v in nums {
            let h_temp: i32 = h_current_max;
            h_current_max = (h_previous_max + v).max(h_current_max);
            h_previous_max = h_temp;
        }
        h_current_max
    }
}

#[test]
fn test() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
}
