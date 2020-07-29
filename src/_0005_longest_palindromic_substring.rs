struct Solution;
use std::iter::FromIterator;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s == "" {
            return s;
        }
        let mut start: usize = 0;
        let mut end: usize = 0;
        let s: Vec<char> = s.chars().collect();

        for i in 0..s.len() {
            let mut left: usize = i;
            let mut right: usize = i;
            while right + 1 < s.len() && s[right + 1] == s[left] {
                right += 1;
            }
            while left > 0 && right < s.len() - 1 && s[left - 1] == s[right + 1] {
                left -= 1;
                right += 1;
            }
            if right - left > end - start {
                start = left;
                end = right;
            }
        }
        String::from_iter(&s[start..=end])
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::longest_palindrome("babad".to_string()),
        "bab".to_string()
    );
    assert_eq!(
        Solution::longest_palindrome("cbbd".to_string()),
        "bb".to_string()
    );
}
