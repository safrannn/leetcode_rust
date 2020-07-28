struct Solution;
impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let n: usize = grid.len();
        let m: usize = grid[0].len();
        let mut h_rotting: Vec<(usize, usize)> = vec![];
        let mut fresh_count: bool = false;
        let mut result: i32 = -1;

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 2 {
                    h_rotting.push((i, j));
                } else if grid[i][j] == 1 {
                    fresh_count = true;
                }
            }
        }
        if !fresh_count {
            return 0;
        }

        while !h_rotting.is_empty() {
            result += 1;
            let mut h_rotting_temp: Vec<(usize, usize)> = vec![];

            while let Some(orange) = h_rotting.pop() {
                if orange.0 > 0 && grid[orange.0 - 1][orange.1] == 1 {
                    h_rotting_temp.push((orange.0 - 1, orange.1));
                    grid[orange.0 - 1][orange.1] = 2;
                }
                if orange.0 < n - 1 && grid[orange.0 + 1][orange.1] == 1 {
                    h_rotting_temp.push((orange.0 + 1, orange.1));
                    grid[orange.0 + 1][orange.1] = 2;
                }
                if orange.1 > 0 && grid[orange.0][orange.1 - 1] == 1 {
                    h_rotting_temp.push((orange.0, orange.1 - 1));
                    grid[orange.0][orange.1 - 1] = 2;
                }
                if orange.1 < m - 1 && grid[orange.0][orange.1 + 1] == 1 {
                    h_rotting_temp.push((orange.0, orange.1 + 1));
                    grid[orange.0][orange.1 + 1] = 2;
                }
            }
            h_rotting = h_rotting_temp;
        }

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    return -1;
                }
            }
        }

        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::oranges_rotting(vec_vec_i32![[2, 1, 1], [1, 1, 0], [0, 1, 1]]),
        4
    );
    assert_eq!(
        Solution::oranges_rotting(vec_vec_i32![[2, 1, 1], [0, 1, 1], [1, 0, 1]]),
        -1
    );
    assert_eq!(Solution::oranges_rotting(vec_vec_i32![[0, 2]]), 0);
}
