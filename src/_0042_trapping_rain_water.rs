struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut max_left_right = vec![0; n];
        let mut max_right_left = vec![0; n];
        let mut cur_max = 0;
        for i in 0..n {
            cur_max = cur_max.max(height[i]);
            max_left_right[i] = cur_max;
        }
        cur_max = 0;
        for i in (0..n).rev() {
            cur_max = cur_max.max(height[i]);
            max_right_left[i] = cur_max;
        }

        let mut result = 0;
        for i in 0..n {
            let surface = max_left_right[i].min(max_right_left[i]);
            result += surface - height[i];
        }

        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    assert_eq!(Solution::trap(vec![]), 0);
}
