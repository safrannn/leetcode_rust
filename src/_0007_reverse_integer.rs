struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let sign = match x < 0 {
            true => -1,
            false => 1,
        };
        let mut result: i32 = 0;
        let mut number: i32 = x.abs();

        while number != 0 {
            match result.checked_mul(10) {
                Some(v) => result = v,
                None => return 0,
            }
            match result.checked_add(number % 10) {
                Some(v) => result = v,
                None => return 0,
            }
            number /= 10;
        }

        result * sign
    }
}

#[test]
fn test() {
    assert_eq!(Solution::reverse(123), 321);
}
