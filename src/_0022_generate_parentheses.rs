struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let mut current: Vec<char> = vec![];
        Self::find(&mut result, &mut current, n, n);
        print!("{:?}", result);
        result
    }

    fn find(result: &mut Vec<String>, current: &mut Vec<char>, left: i32, right: i32) {
        if left == 0 && right == 0 {
            result.push(current.iter().collect::<String>());
        } else {
            if left > 0 {
                current.push('(');
                Self::find(result, current, left - 1, right);
                current.pop();
            }
            if right > left {
                current.push(')');
                Self::find(result, current, left, right - 1);
                current.pop();
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::generate_parenthesis(3),
        vec_string!["((()))", "(()())", "(())()", "()(())", "()()()"]
    );
}
