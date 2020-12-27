struct Solution;

impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();
        let mut result = vec![-1; n];

        for b in 0..n {
            let mut i = 0;
            let mut j = b;
            while i < m {
                if grid[i][j] == 1 {
                    if j + 1 < n && grid[i][j + 1] == grid[i][j] {
                        i += 1;
                        j += 1;
                    } else {
                        break;
                    }
                } else {
                    if j >= 1 && grid[i][j - 1] == grid[i][j] {
                        i += 1;
                        j -= 1;
                    } else {
                        break;
                    }
                }
            }
            if i == m {
                result[b] = j as i32;
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::find_ball(vec_vec_i32![
            [1, 1, 1, -1, -1],
            [1, 1, 1, -1, -1],
            [-1, -1, -1, 1, 1],
            [1, 1, 1, 1, -1],
            [-1, -1, -1, -1, -1]
        ]),
        vec![1, -1, -1, -1, -1]
    );
    assert_eq!(Solution::find_ball(vec_vec_i32![[-1]]), vec![-1]);
}
