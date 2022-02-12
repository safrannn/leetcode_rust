struct Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let (mut prev, mut cur, mut result) = (0, 1, 0);
        let chars = s.chars().collect::<Vec<char>>();
        for i in 1..s.len() {
            if &chars[i] == &chars[i - 1] {
                cur += 1;
            } else {
                result += prev.min(cur);
                prev = cur;
                cur = 1;
            }
        }
        result += prev.min(cur);
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_binary_substrings("00110011".to_string()), 6);
    assert_eq!(Solution::count_binary_substrings("10101".to_string()), 4);
}
