struct Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let (mut num, mut count) = (num, 0);
        while num > 0 {
            if num & 1 == 1 {
                num -= 1;
            } else {
                num >>= 1;
            }
            count += 1;
        }
        count
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::number_of_steps(14), 6);
}

#[test]
fn test_2() {
    assert_eq!(Solution::number_of_steps(8), 4);
}

#[test]
fn test_3() {
    assert_eq!(Solution::number_of_steps(123), 12);
}
