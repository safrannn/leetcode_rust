struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut carry: i32 = 1;
        let mut sum: i32;
        let mut result: Vec<i32> = Vec::new();
        for v in digits.iter().rev() {
            sum = carry + v;
            carry = sum / 10;
            sum = sum % 10;
            result.insert(0, sum);
        }
        if carry != 0 {
            result.insert(0, carry);
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
}
