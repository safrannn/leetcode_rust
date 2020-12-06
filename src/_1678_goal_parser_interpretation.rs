struct Solution;
impl Solution {
    pub fn interpret(command: String) -> String {
        command.replace("()", "o").replace("(al)", "al")
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::max_operations("G()(al)".to_string()),
        "Goal".to_string()
    );
    assert_eq!(
        Solution::max_operations("G()()()()(al)".to_string()),
        "Gooooal".to_string()
    );
    assert_eq!(
        Solution::max_operations("(al)G(al)()()G".to_string()),
        "alGalooG".to_string()
    );
}
