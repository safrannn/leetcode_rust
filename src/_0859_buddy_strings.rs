struct Solution;
impl Solution {
    pub fn buddy_strings(a: String, b: String) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let n = a.len();
        let a = a.as_bytes();
        let b = b.as_bytes();
        if a == b {
            let mut ocur = vec![0; 26];
            for i in 0..n {
                ocur[a[i] as usize - 'a' as usize] += 1;
            }
            for i in 0..26 {
                if ocur[i] >= 2 {
                    return true;
                }
            }
            return false;
        } else {
            let mut diffs: Vec<(u8, u8)> = vec![];
            for i in 0..n {
                if a[i] != b[i] {
                    diffs.push((a[i].clone(), b[i].clone()));
                }
            }
            if diffs.len() != 2 || !(diffs[0].0 == diffs[1].1 && diffs[1].0 == diffs[0].1) {
                return false;
            } else {
                return true;
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::buddy_strings("ab".to_string(), "ba".to_string()),
        true
    );
    assert_eq!(
        Solution::buddy_strings("ab".to_string(), "ab".to_string()),
        false
    );
    assert_eq!(
        Solution::buddy_strings("aa".to_string(), "aa".to_string()),
        true
    );
    assert_eq!(
        Solution::buddy_strings("aaaaaaabc".to_string(), "aaaaaaacb".to_string()),
        true
    );
    assert_eq!(
        Solution::buddy_strings("".to_string(), "aa".to_string()),
        false
    );
    assert_eq!(
        Solution::buddy_strings("abac".to_string(), "abad".to_string()),
        false
    );
}
