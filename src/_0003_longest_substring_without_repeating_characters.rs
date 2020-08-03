struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_index: Vec<usize> = vec![0; 128];
        let mut result: usize = 0;
        let mut start: usize = 0;
        for (i, v) in s.chars().enumerate() {
            start = start.max(last_index[v as usize - ' ' as usize]);
            result = result.max(i - start + 1);
            last_index[v as usize - ' ' as usize] = i + 1;
        }
        result as i32
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::length_of_longest_substring("abcabcbb".to_string()),
        3
    );
    assert_eq!(
        Solution::length_of_longest_substring("bbbbb".to_string()),
        1
    );
    assert_eq!(
        Solution::length_of_longest_substring("pwwkew".to_string()),
        3
    );
}
