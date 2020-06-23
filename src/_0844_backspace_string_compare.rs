struct Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let s_ = remove_backspace(s);
        let t_ = remove_backspace(t);
        s_ == t_
    }
}

fn remove_backspace(s: String) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    for v in s.chars() {
        if v == '#' {
            result.pop();
        } else {
            result.push(v);
        }
    }
    result
}

#[test]
fn test() {
    assert_eq!(
        Solution::backspace_compare("ab#c".to_string(), "ad#c".to_string()),
        true
    );
    assert_eq!(
        Solution::backspace_compare("ab##".to_string(), "c#d#".to_string()),
        true
    );
    assert_eq!(
        Solution::backspace_compare("a##c".to_string(), "#a#c".to_string()),
        true
    );
    assert_eq!(
        Solution::backspace_compare("a#c".to_string(), "b".to_string()),
        false
    );
}
