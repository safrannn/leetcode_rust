struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut characters = vec![0 as i32; 26];
        for v in magazine.chars() {
            characters[(v as u8 - 'a' as u8) as usize] += 1;
        }
        for v in ransom_note.chars() {
            if characters[(v as u8 - 'a' as u8) as usize] == 0 {
                return false;
            }
            characters[(v as u8 - 'a' as u8) as usize] -= 1;
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::can_construct("a".to_string(), "b".to_string()),
        false
    );
    assert_eq!(
        Solution::can_construct("aa".to_string(), "ab".to_string()),
        false
    );
    assert_eq!(
        Solution::can_construct("aa".to_string(), "aab".to_string()),
        true
    );
}
