struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut result = vec![];
        for v in s.split_whitespace().collect::<Vec<&str>>() {
            result.push(v.chars().rev().collect::<String>());
        }
        result.join(" ")
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::reverse_words("Let's take LeetCode contest".to_string()),
        "s'teL ekat edoCteeL tsetnoc".to_string()
    );
}
