struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let mut char_count: usize = 0;
        let mut h_stack: Vec<(usize, Vec<char>)> = vec![];
        let mut result: Vec<char> = vec![];
        let n: usize = s.len();

        for i in 0..n {
            let c_cur: char = s[i].clone();
            if c_cur.is_ascii_digit() {
                char_count = char_count * 10 + (c_cur as usize - '0' as usize);
            } else if c_cur == '[' {
                h_stack.push((char_count, vec![]));
                char_count = 0;
            } else if c_cur == ']' {
                if let Some(h_last) = h_stack.pop() {
                    for _k in 0..h_last.0 {
                        if let Some(current) = h_stack.last_mut() {
                            current.1.extend_from_slice(&h_last.1);
                        } else {
                            result.extend_from_slice(&h_last.1);
                        }
                    }
                }
            } else {
                if let Some(current) = h_stack.last_mut() {
                    current.1.push(c_cur);
                } else {
                    result.push(c_cur);
                }
            }
        }
        result.into_iter().collect::<String>()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::decode_string("3[a]2[bc]".to_string()),
        "aaabcbc".to_string()
    );
    assert_eq!(
        Solution::decode_string("3[a2[c]]".to_string()),
        "accaccacc".to_string()
    );
    assert_eq!(
        Solution::decode_string("2[abc]3[cd]ef".to_string()),
        "abcabccdcdcdef".to_string()
    );
    assert_eq!(
        Solution::decode_string("abc3[cd]xyz".to_string()),
        "abccdcdcdxyz".to_string()
    );
    assert_eq!(
        Solution::decode_string("3[2[a]]".to_string()),
        "aaaaaa".to_string()
    );
}
