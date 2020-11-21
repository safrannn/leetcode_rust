struct Solution;

impl Solution {
    pub fn sorted_squares(mut a: Vec<i32>) -> Vec<i32> {
        a.iter_mut().for_each(|s| {
            *s *= *s;
        });
        a.sort();
        a
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
