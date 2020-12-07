struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let direction: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut result = vec![];

        let mut cur_pos = (0, 0);
        let mut margin: (i32, i32, i32, i32) =
            (0, matrix.len() as i32 - 1, 0, matrix[0].len() as i32 - 1); // up, bottom, left, right
        let mut cur_dir: usize = 0;
        let mut visited = 0;

        while visited < matrix[0].len() * matrix.len() {
            result.push(matrix[cur_pos.0][cur_pos.1]);
            visited += 1;

            let next_pos: (i32, i32) = (
                cur_pos.0 as i32 + direction[cur_dir].0,
                cur_pos.1 as i32 + direction[cur_dir].1,
            );
            if (next_pos.0 < margin.0 || next_pos.0 > margin.1)
                || (next_pos.1 < margin.2 || next_pos.1 > margin.3)
            {
                cur_dir = if cur_dir == 3 { 0 } else { cur_dir + 1 };
                match cur_dir {
                    0 => margin.2 += 1,
                    1 => margin.0 += 1,
                    2 => margin.3 -= 1,
                    3 => margin.1 -= 1,
                    _ => {}
                }
            }
            cur_pos = (
                (cur_pos.0 as i32 + direction[cur_dir].0) as usize,
                (cur_pos.1 as i32 + direction[cur_dir].1) as usize,
            );
        }

        result
    }
}

#[test]
fn test() {
    let matrix: Vec<Vec<i32>> = vec_vec_i32![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let res = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
    assert_eq!(Solution::spiral_order(matrix), res);
    let matrix: Vec<Vec<i32>> = vec_vec_i32![[3], [2]];
    let res = vec![3, 2];
    assert_eq!(Solution::spiral_order(matrix), res);
}
