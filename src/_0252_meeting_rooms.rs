struct Solution;

impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        intervals.sort_unstable();
        for i in 1..intervals.len() {
            if intervals[i - 1][1] > intervals[i][0] {
                return false;
            }
        }
        return true;
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::can_attend_meetings(vec_vec_i32![[0, 30], [5, 10], [15, 20]]),
        false
    );
    assert_eq!(
        Solution::can_attend_meetings(vec_vec_i32![[7, 10], [2, 4]]),
        true
    );
}
