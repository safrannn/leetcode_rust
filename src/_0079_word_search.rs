struct Solution;
// use rustgym::util::*;

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let chars: Vec<char> = word.chars().collect();
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == chars[0] && Self::find(&mut board, &chars, 1, i, j) {
                    return true;
                }
            }
        }
        false
    }

    fn find(
        board: &mut Vec<Vec<char>>,
        chars: &Vec<char>,
        char_check_i: usize,
        i: usize,
        j: usize,
    ) -> bool {
        if char_check_i >= chars.len() {
            return true;
        }
        let char_check: char = chars[char_check_i];
        let mut result: bool = false;
        let m: usize = board.len();
        let n: usize = board[0].len();
        board[i][j] = ' ';

        if i > 0 && board[i - 1][j] == char_check {
            result = Self::find(board, chars, char_check_i + 1, i - 1, j);
        }
        if !result && i < m - 1 && board[i + 1][j] == char_check {
            result = Self::find(board, chars, char_check_i + 1, i + 1, j);
        }
        if !result && j > 0 && board[i][j - 1] == char_check {
            result = Self::find(board, chars, char_check_i + 1, i, j - 1);
        }
        if !result && j < n - 1 && board[i][j + 1] == char_check {
            result = Self::find(board, chars, char_check_i + 1, i, j + 1);
        }
        board[i][j] = chars[char_check_i - 1];
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::exist(
            vec_vec_char![
                ['A', 'B', 'C', 'E'],
                ['S', 'F', 'C', 'S'],
                ['A', 'D', 'E', 'E']
            ],
            "ABCCED".to_string()
        ),
        true
    );
    assert_eq!(
        Solution::exist(
            vec_vec_char![
                ['A', 'B', 'C', 'E'],
                ['S', 'F', 'C', 'S'],
                ['A', 'D', 'E', 'E']
            ],
            "SEE".to_string()
        ),
        true
    );
    assert_eq!(
        Solution::exist(
            vec_vec_char![
                ['A', 'B', 'C', 'E'],
                ['S', 'F', 'C', 'S'],
                ['A', 'D', 'E', 'E']
            ],
            "ABCB".to_string()
        ),
        false
    );
}
