struct Solution;

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut mem: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; 26]; n]; n + 1];

        for length in 2..=n {
            for start in 0..=n - length {
                for k in 0..26 {
                    let c = s[start] as usize - 'a' as usize;
                    if s[start] == s[start + length - 1] && c != k {
                        mem[length][start][c] =
                            mem[length][start][c].max(mem[length - 2][start + 1][k] + 2);
                    }
                    mem[length][start][k] = mem[length][start][k].max(mem[length - 1][start][k]);
                    mem[length][start][k] =
                        mem[length][start][k].max(mem[length - 1][start + 1][k]);
                }
            }
        }

        let mut result = 0;
        for v in &mem[n][0] {
            result = result.max(v.clone());
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::longest_palindrome_subseq("bbabab".to_string()), 4);
    assert_eq!(
        Solution::longest_palindrome_subseq("dcbccacdb".to_string()),
        4
    );
}
