struct Solution;
use std::cmp::Reverse;

impl Solution {
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by_key(|v| (v[0], Reverse(v[1])));

        let mut result = 0;
        let mut v_1 = -1;
        for inter in intervals {
            if inter[1] > v_1 {
                result += 1;
                v_1 = inter[1];
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::remove_covered_intervals(vec_vec_i32![[1, 4], [3, 6], [2, 8]]),
        2
    );
    assert_eq!(
        Solution::remove_covered_intervals(vec_vec_i32![[1, 4], [2, 3]]),
        1
    );
    assert_eq!(
        Solution::remove_covered_intervals(vec_vec_i32![[0, 10], [5, 12]]),
        2
    );
    assert_eq!(
        Solution::remove_covered_intervals(vec_vec_i32![[3, 10], [4, 10], [5, 11]]),
        2
    );
    assert_eq!(
        Solution::remove_covered_intervals(vec_vec_i32![[1, 2], [1, 4], [3, 4]]),
        1
    );
}
