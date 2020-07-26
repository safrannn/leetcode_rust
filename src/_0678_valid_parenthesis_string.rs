struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut h_low: i32 = 0;
        let mut h_high: i32 = 0;
        for v in s.chars() {
            if v == '(' {
                h_low += 1;
            } else {
                h_low -= 1;
            }

            if v != ')' {
                h_high += 1;
            } else {
                h_high -= 1;
            }

            if h_high < 0 {
                return false;
            }
            h_low = h_low.max(0);
        }
        h_low == 0
    }
}

#[test]
fn test() {
    assert_eq!(Solution::check_valid_string("()".to_string()), true);
    assert_eq!(Solution::check_valid_string("(*)".to_string()), true);
    assert_eq!(Solution::check_valid_string("(*))".to_string()), true);
}
