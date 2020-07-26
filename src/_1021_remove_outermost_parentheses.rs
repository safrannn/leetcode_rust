struct Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut result: String = "".to_string();
        let mut count: i32 = 0;
        for v in s.chars() {
            match v {
                '(' => {
                    count += 1;
                    if count > 1 {
                        result.push(v);
                    }
                }
                ')' => {
                    count -= 1;
                    if count != 0 {
                        result.push(v);
                    }
                }
                _ => {}
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::remove_outer_parentheses("(()())(())".to_string(),),
        "()()()".to_string()
    );
    assert_eq!(
        Solution::remove_outer_parentheses("(()())(())(()(()))".to_string(),),
        "()()()()(())".to_string()
    );
    assert_eq!(
        Solution::remove_outer_parentheses("()()".to_string(),),
        "".to_string()
    );
}
