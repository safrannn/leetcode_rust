struct Solution;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut h_char_index: Vec<i32> = vec![-1; 26];
        for (i, v) in s.chars().enumerate() {
            if h_char_index[v as usize - 'a' as usize] >= 0 {
                h_char_index[v as usize - 'a' as usize] = -2;
            } else if h_char_index[v as usize - 'a' as usize] == -1 {
                h_char_index[v as usize - 'a' as usize] = i as i32;
            }
        }
        let mut result: i32 = s.len() as i32;
        for v in h_char_index {
            if v >= 0 {
                result = result.min(v);
            }
        }
        if result == s.len() as i32 {
            return -1;
        } else {
            return result;
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
    assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
}
