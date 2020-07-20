struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_checker: Vec<Vec<bool>> = vec![vec![false; 9]; 9];
        let mut col_checker: Vec<Vec<bool>> = vec![vec![false; 9]; 9];
        let mut box_checker: Vec<Vec<bool>> = vec![vec![false; 9]; 9];

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                let character: char = board[i][j];
                if character == '.' {
                    continue;
                }
                let number: usize = character as usize - b'1' as usize;
                if row_checker[i][number] {
                    return false;
                } else {
                    row_checker[i][number] = true;
                }
                if col_checker[j][number] {
                    return false;
                } else {
                    col_checker[j][number] = true;
                }
                let k: usize = (i / 3) * 3 + j / 3;
                if box_checker[k][number] {
                    return false;
                } else {
                    box_checker[k][number] = true;
                }
            }
        }
        true
    }
}

#[test]
fn test() {
    let board = vec_vec_char![
        ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        ['.', '.', '.', '.', '8', '.', '.', '7', '9']
    ];
    assert_eq!(Solution::is_valid_sudoku(board), true);
    let board = vec_vec_char![
        ['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        ['.', '.', '.', '.', '8', '.', '.', '7', '9']
    ];
    assert_eq!(Solution::is_valid_sudoku(board), false);
}
