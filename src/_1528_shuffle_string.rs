struct Solution;

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let s_char: Vec<char> = s.chars().collect();
        let mut result: Vec<char> = vec!['a'; s.len()];

        for (i, v) in indices.into_iter().enumerate() {
            result[v as usize] = s_char[i];
        }
        result.into_iter().collect()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::restore_string("abc".to_string(), vec![0, 1, 2]),
        "abc".to_string()
    );
    assert_eq!(
        Solution::restore_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3]),
        "leetcode".to_string()
    );
    assert_eq!(
        Solution::restore_string("aiohn".to_string(), vec![3, 1, 4, 2, 0]),
        "nihao".to_string()
    );
    assert_eq!(
        Solution::restore_string("aaiougrt".to_string(), vec![4, 0, 2, 6, 7, 3, 1, 5]),
        "arigatou".to_string()
    );
    assert_eq!(
        Solution::restore_string("art".to_string(), vec![1, 0, 2]),
        "rat".to_string()
    );
}
