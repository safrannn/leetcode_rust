struct Solution;

impl Solution {
    pub fn rotate_string(a: String, b: String) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut pool = "".to_string();
        pool += &a;
        pool += &a;
        pool.contains(&b)
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::rotate_string("abcde".to_string(), "cdeab".to_string()),
        true
    );
    assert_eq!(
        Solution::rotate_string("abcde".to_string(), "abced".to_string()),
        false
    );
}
