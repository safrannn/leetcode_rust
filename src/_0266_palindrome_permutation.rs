struct Solution;

impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
        let mut count = vec![0 as u8; 256];
        for v in s.chars() {
            count[(v as u8 - 'a' as u8) as usize] += 1;
        }
        let mut switch = false;
        for v in count {
            if v % 2 != 0 {
                if switch {
                    return false;
                } else {
                    switch = true;
                }
            }
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(Solution::can_permute_palindrome("code".to_string()), false);
    assert_eq!(Solution::can_permute_palindrome("aab".to_string()), true);
    assert_eq!(
        Solution::can_permute_palindrome("carerac".to_string()),
        true
    );
}
