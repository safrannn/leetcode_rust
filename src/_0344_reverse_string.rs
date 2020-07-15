struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.len() < 2 {
            return;
        }

        let last_index = s.len() - 1;

        for index in 0..last_index / 2 + 1 {
            let tmp = s[index];
            s[index] = s[last_index - index];
            s[last_index - index] = tmp;
        }
    }
}

#[test]
fn test() {
    let mut input: Vec<char> = vec![];
    let output: Vec<char> = vec![];
    Solution::reverse_string(&mut input);
    assert_eq!(input, output);
    let mut input: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
    let output: Vec<char> = vec!['o', 'l', 'l', 'e', 'h'];
    Solution::reverse_string(&mut input);
    assert_eq!(input, output);
}
