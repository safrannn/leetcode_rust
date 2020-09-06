struct Solution;
impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut result: Vec<char> = vec![];
        let mut char_fill: u8 = 0;
        let s: Vec<char> = s.chars().collect();
        for (i, v) in s.iter().enumerate() {
            match v {
                '?' => {
                    let mut potential_char = (char_fill + b'a') as char;
                    while (i > 0 && potential_char == result[i - 1])
                        || (i < s.len() - 1 && potential_char == s[i + 1])
                    {
                        if char_fill == 27 {
                            char_fill = 0;
                        }
                        char_fill += 1;
                        potential_char = (char_fill + b'a') as char;
                    }
                    result.push(potential_char);
                    if char_fill == 27 {
                        char_fill = 0;
                    }
                    char_fill += 1;
                }
                _ => result.push(v.clone()),
            }
        }
        dbg!(&result);
        result.into_iter().collect()
    }
}
#[test]
fn test() {
    assert_eq!(
        Solution::modify_string("?zs".to_string()),
        "azs".to_string()
    );
    assert_eq!(
        Solution::modify_string("ubv?w".to_string()),
        "ubvaw".to_string()
    );
    assert_eq!(
        Solution::modify_string("j?qg??b".to_string()),
        "jaqgbcb".to_string()
    );
    assert_eq!(
        Solution::modify_string("??yw?ipkj?".to_string()),
        "abywcipkjd".to_string()
    );
}
