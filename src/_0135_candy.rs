struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut left_right = vec![1; n];
        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                left_right[i] = left_right[i - 1] + 1;
            }
        }
        let mut right_left = vec![1; n];
        for i in (1..n).rev() {
            if ratings[i - 1] > ratings[i] {
                right_left[i - 1] = right_left[i] + 1;
            }
        }

        let mut result = vec![0; n];
        for i in 0..n {
            result[i] = left_right[i].max(right_left[i]);
        }
        result.iter().sum()
    }
}

fn test() {
    assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
    assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
    assert_eq!(Solution::candy(vec![1, 2, 87, 87, 87, 2, 1]), 13);
}
