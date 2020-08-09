struct Solution;

impl Solution {
    pub fn can_convert_string(s: String, t: String, k: i32) -> bool {
        let mut h_shift: Vec<i32> = vec![-1; 26];
        let n_s = s.len();
        let n_t = t.len();
        let s_char: Vec<char> = s.chars().collect();
        let t_char: Vec<char> = t.chars().collect();
        if n_s != n_t {
            return false;
        }

        for i in 0..n_s {
            if s_char[i] == t_char[i] {
                continue;
            } else {
                let mut h_diff = t_char[i] as i32 - s_char[i] as i32;
                if h_diff < 0 {
                    h_diff += 26;
                }
                let h_diff: usize = h_diff as usize % 26;
                if h_shift[h_diff] > 0 {
                    h_shift[h_diff] += 26;
                } else {
                    h_shift[h_diff] = h_diff as i32;
                }
                if h_shift[h_diff] > k {
                    return false;
                }
            }
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::can_convert_string("input".to_string(), "output".to_string(), 9),
        true
    );
    assert_eq!(
        Solution::can_convert_string("abc".to_string(), "bcd".to_string(), 10),
        false
    );
    assert_eq!(
        Solution::can_convert_string("aab".to_string(), "bbb".to_string(), 27),
        true
    );
}
