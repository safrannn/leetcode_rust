struct Solution;
impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let n: usize = mat.len();
        let mut result: i32 = 0;
        for i in 0..n {
            for j in 0..n {
                if i == j || i == n - 1 - j {
                    result += mat[i][j];
                }
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::diagonal_sum(vec_vec_i32![[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
        25
    );
    assert_eq!(
        Solution::diagonal_sum(vec_vec_i32![
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1]
        ]),
        8
    );
    assert_eq!(Solution::diagonal_sum(vec_vec_i32![[5]]), 5);
}
