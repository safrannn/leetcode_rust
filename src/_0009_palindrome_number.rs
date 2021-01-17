struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut reversed = 0;
        let mut number = x;
        while number > 0 {
            reversed = reversed * 10 + number % 10;
            number /= 10;
        }
        x == reversed
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_palindrome(121), true);
    assert_eq!(Solution::is_palindrome(-121), false);
    assert_eq!(Solution::is_palindrome(10), false);
}
