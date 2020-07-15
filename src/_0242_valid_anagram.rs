struct Solution;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut char_count: Vec<i32> = vec![0; 26];
        let a_offset = 'a' as usize;
        for ch in s.chars() {
            char_count[ch as usize - a_offset] += 1;
        }

        for ch in t.chars() {
            char_count[ch as usize - a_offset] -= 1;
        }

        for v in char_count {
            if v != 0 {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
        true
    );
}
