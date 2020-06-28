struct Solution;

impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut left: usize = 0;
        let mut right: usize = a.len() - 1;

        let mut result: Vec<i32> = vec![0; a.len()];
        let mut i: usize = a.len() - 1;

        let mut left_result: i32;
        let mut right_result: i32;

        while left <= right {
            left_result = a[left] * a[left];
            right_result = a[right] * a[right];

            if left_result >= right_result {
                result[i] = left_result;
                left += 1;
            } else {
                result[i] = right_result;
                right -= 1;
            }
            if i > 0 {
                i -= 1;
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
        vec![0, 1, 9, 16, 100]
    );
    assert_eq!(
        Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
        vec![4, 9, 9, 49, 121]
    );
    assert_eq!(Solution::sorted_squares(vec![1]), vec![1]);
}
