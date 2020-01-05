struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut fast = n;
        let mut slow = n;
        loop {
            slow = Self::square_sum(slow);
            fast = Self::square_sum(fast);
            fast = Self::square_sum(fast);

            if slow == 1 || fast == 1 {
                return true;
            }

            if slow == fast {
                return false;
            }
        }
    }

    fn square_sum(mut n: i32) -> i32 {
        let mut result = 0;
        while n > 0 {
            let digit = n % 10;
            result += digit * digit;
            n /= 10;
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_happy(19), true);

    assert_eq!(Solution::is_happy(1), true);
}
