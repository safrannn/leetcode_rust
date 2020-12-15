struct Solution;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut allowed_chars: Vec<bool> = vec![false; 26];
        for v in allowed.chars() {
            allowed_chars[v as usize - 'a' as usize] = true;
        }

        let mut result = 0;
        for word in words {
            let mut same = true;
            for v in word.chars() {
                if !allowed_chars[v as usize - 'a' as usize] {
                    same = false;
                    break;
                };
            }
            if same {
                result += 1;
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::count_consistent_strings(
            "ab".to_string(),
            vec_string!["ad", "bd", "aaab", "baa", "badab"]
        ),
        2
    );
    assert_eq!(
        Solution::count_consistent_strings(
            "abc".to_string(),
            vec_string!["a", "b", "c", "ab", "ac", "bc", "abc"]
        ),
        7
    );
    assert_eq!(
        Solution::count_consistent_strings(
            "cad".to_string(),
            vec_string!["cc", "acd", "b", "ba", "bac", "bad", "ac", "d"]
        ),
        4
    );
}
