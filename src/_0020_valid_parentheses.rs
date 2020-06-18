struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut char_stack: Vec<char> = vec![];

        for c in s.chars() {
            match c {
                '(' | '{' | '[' => char_stack.push(c),
                ')' | '}' | ']' => match char_stack.pop() {
                    Some(l) => {
                        if !((c == ')' && l == '(')
                            | (c == '}' && l == '{')
                            | (c == ']' && l == '['))
                        {
                            return false;
                        }
                    }
                    None => return false,
                },
                _ => {}
            }
        }

        char_stack.is_empty()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_valid(String::from("()")), true);
    assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
    assert_eq!(Solution::is_valid(String::from("([)]")), false);
    assert_eq!(Solution::is_valid(String::from("{[]}")), true);
}
