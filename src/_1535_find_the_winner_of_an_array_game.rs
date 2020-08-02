struct Solution;

impl Solution {
    pub fn get_winner(arr: Vec<i32>, mut k: i32) -> i32 {
        let mut h_current_max: i32 = arr[0];
        let mut h_compare_count: i32 = 0;
        let mut h_compare_index: usize = 1;
        if k > arr.len() as i32 {
            k = arr.len() as i32;
        }
        while h_compare_index < arr.len() {
            if arr[h_compare_index] > h_current_max {
                h_current_max = arr[h_compare_index];
                h_compare_count = 1;
            } else {
                h_compare_count += 1;
            }
            if h_compare_count == k {
                return h_current_max;
            }
            h_compare_index += 1;
        }
        h_current_max
    }
}

#[test]
fn test() {
    assert_eq!(Solution::get_winner(vec![2, 1, 3, 5, 4, 6, 7], 2), 5);
}
