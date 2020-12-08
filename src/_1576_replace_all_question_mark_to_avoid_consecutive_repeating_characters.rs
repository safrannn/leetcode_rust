struct Solution;
impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut result = s.as_bytes().to_owned();
        let mut ch = 0;

        for i in 0..s.len() {
            if result[i] == '?' as u8 {
                if i == 0 {
                    if s.len() != 1 && result[1] == 'a' as u8 {
                        ch += 1;
                    }
                } else if i == s.len() - 1 {
                    ch = (ch + 1) % 26;
                } else {
                    while result[i + 1] == 'a' as u8 + ch || result[i - 1] == 'a' as u8 + ch {
                        ch = (ch + 1) % 26;
                    }
                }

                result[i] = 'a' as u8 + ch;
            } else {
                ch = (result[i] as u8 - 'a' as u8 + 1) % 26;
            }
        }

        std::str::from_utf8(&result).unwrap().to_string()
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
        "ubvxw".to_string()
    );
    assert_eq!(
        Solution::modify_string("j?qg??b".to_string()),
        "jkqghib".to_string()
    );
    assert_eq!(
        Solution::modify_string("??yw?ipkj?".to_string()),
        "abywxipkjl".to_string()
    );
}
