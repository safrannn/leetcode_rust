struct Solution;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut characters: Vec<char> = s.chars().collect::<Vec<char>>();
        let mut h_stack: Vec<usize> = vec![];
        for i in 0..characters.len() {
            match characters[i] {
                ')' => match h_stack.pop() {
                    None => characters[i] = '_',
                    _ => {}
                },
                '(' => {
                    h_stack.push(i);
                }
                _ => {}
            }
        }
        while let Some(index) = h_stack.pop() {
            characters[index] = '_';
        }
        characters.iter().filter(|&x| *x != '_').collect::<String>()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::min_remove_to_make_valid("lee(t(c)o)de)".to_string()),
        "lee(t(c)o)de".to_string()
    );
    assert_eq!(
        Solution::min_remove_to_make_valid("a)b(c)d".to_string()),
        "ab(c)d".to_string()
    );
    assert_eq!(
        Solution::min_remove_to_make_valid("))((".to_string()),
        "".to_string()
    );
    assert_eq!(
        Solution::min_remove_to_make_valid("(a(b(c)d)".to_string()),
        "a(b(c)d)".to_string()
    );
}
