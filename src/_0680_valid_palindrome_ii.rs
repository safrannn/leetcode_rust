struct Solution;

fn is_palindrome(s: &str) -> bool {
    for (a, b) in s.chars().zip(s.chars().rev()) {
        if a != b {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let mut l: usize = 0;
        let mut r: usize = s.len() - 1;
        while l < r {
            if s[l..=l] == s[r..=r] {
                l += 1;
                r -= 1;
            } else if is_palindrome(&s[l..r]) {
                return true;
            } else if is_palindrome(&s[(l + 1)..=r]) {
                return true;
            } else {
                return false;
            }
        }
        true
    }
}
#[test]
fn test() {
    let s = "aba".to_string();
    assert_eq!(Solution::valid_palindrome(s), true);
    let s = "abca".to_string();
    assert_eq!(Solution::valid_palindrome(s), true);
    let s = "accca".to_string();
    assert_eq!(Solution::valid_palindrome(s), true);
}
