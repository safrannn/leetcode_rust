struct Solution;

impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        let mut h_status: Vec<Vec<i32>> = vec![];
        for interval in intervals {
            h_status.push(vec![interval[0], 1]);
            h_status.push(vec![interval[1], -1]);
        }
        h_status.sort_unstable();

        let mut room: i32 = 0;
        let mut h_max: i32 = 0;
        for v in h_status {
            room += v[1];
            h_max = i32::max(room, h_max);
        }
        h_max
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::min_meeting_rooms(vec_vec_i32![[7, 10], [2, 4]]),
        1
    );
    assert_eq!(
        Solution::min_meeting_rooms(vec_vec_i32![[0, 30], [5, 10], [15, 20]]),
        2
    );
}
