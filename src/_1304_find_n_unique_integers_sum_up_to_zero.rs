struct Solution;

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(n as usize);
        if n & 1 == 1 {
            result.push(0);
        }
        for i in 1..=n / 2 {
            result.push(i);
            result.push(-i)
        }
        result
    }
}

#[test]
fn test() {
    let arr1 = vec![0, 1, -1, 2, -2];
    assert_eq!(Solution::sum_zero(5), arr1);
}
