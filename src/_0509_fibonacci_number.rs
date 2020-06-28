struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut num1: i32 = 0;
        let mut num2: i32 = 1;
        for _ in 0..n {
            num1 += num2;
            std::mem::swap(&mut num1, &mut num2);
        }
        num1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::fib(2), 1);
    assert_eq!(Solution::fib(3), 2);
    assert_eq!(Solution::fib(4), 3);
}
