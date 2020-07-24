struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s_chars: Vec<char> = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        let compare_1: String = s_chars.iter().collect();
        let compare_2: String = s_chars.iter().rev().collect();
        compare_1 == compare_2
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
        true
    );
    assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
}
