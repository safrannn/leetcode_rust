struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let look_up = |c: &char| -> i32 {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        };

        let chars = s.chars().collect::<Vec<char>>();
        let mut prev = 'I';
        let mut result = 0;
        for i in (0..s.len()).rev() {
            let c = &chars[i];
            if look_up(c) < look_up(&prev) {
                result -= look_up(c);
            } else {
                result += look_up(c);
                prev = *c;
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
}
