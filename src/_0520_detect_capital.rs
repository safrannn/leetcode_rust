struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        if word == word.to_uppercase() || word == word.to_lowercase() {
            return true;
        }
        let n: usize = word.len();
        if word.chars().next().unwrap().is_ascii_uppercase()
            && word[1..n] == word.to_lowercase()[1..n]
        {
            return true;
        }
        false
    }
}

#[test]
fn test() {
    assert_eq!(Solution::detect_capital_use("USA".to_string()), true);
    assert_eq!(Solution::detect_capital_use("FlaG".to_string()), false);
    assert_eq!(Solution::detect_capital_use("leetcode".to_string()), true);
}
