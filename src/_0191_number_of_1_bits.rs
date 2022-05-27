struct Solution;

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let (mut n, mut count) = (n, 0);
        while n > 0 {
            count += if n & 1 == 1 { 1 } else { 0 };
            n >>= 1;
        }
        count
    }
}
#[test]
fn test_1() {
    assert_eq!(Solution::hammingWeight(00000000000000000000000000001011), 3);
}

#[test]
fn test_2() {
    assert_eq!(Solution::hammingWeight(00000000000000000000000010000000), 1);
}

#[test]
fn test_3() {
    assert_eq!(
        Solution::hammingWeight(11111111111111111111111111111101),
        31
    );
}
