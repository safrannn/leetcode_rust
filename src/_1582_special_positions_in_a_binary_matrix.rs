struct Solution;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut row_counts: Vec<i32> = vec![0; mat.len()];
        let mut col_counts: Vec<i32> = vec![0; mat[0].len()];
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                let current = mat[i][j];
                row_counts[i] += current;
                col_counts[j] += current;
            }
        }
        let mut result = 0;
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                if row_counts[i] == 1 && col_counts[j] == 1 && mat[i][j] == 1 {
                    result += 1;
                }
            }
        }
        result
    }
}
#[test]
fn test() {
    assert_eq!(
        Solution::num_special(vec_vec_i32![[1, 0, 0], [0, 0, 1], [1, 0, 0]]),
        1
    );
    assert_eq!(
        Solution::num_special(vec_vec_i32![[1, 0, 0], [0, 1, 0], [0, 0, 1]]),
        3
    );
    assert_eq!(
        Solution::num_special(vec_vec_i32![
            [0, 0, 0, 1],
            [1, 0, 0, 0],
            [0, 1, 1, 0],
            [0, 0, 0, 0]
        ]),
        2
    );
    assert_eq!(
        Solution::num_special(vec_vec_i32![
            [0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 0, 1, 1]
        ]),
        3
    );
}
