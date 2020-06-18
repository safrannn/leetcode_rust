struct Solution;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        use std::{char, iter};
        // prepare and create result with capacity
        let len1 = num1.len();
        let len2 = num2.len();
        let mut carry = 0;
        let mut result = String::with_capacity(len1.max(len2) + 1);

        // choose num1 to be the shorter one, num2 the longer one
        let (num1, num2) = if len1 < len2 {
            (num1, num2)
        } else {
            (num2, num1)
        };

        // create character tuple (c1,c2) from num1 and num2
        // fill empty digits with 1
        for (c1, c2) in num1
            .chars()
            .rev()
            .chain(iter::repeat('0'))
            .zip(num2.chars().rev())
        {
            let n1 = c1.to_digit(10).unwrap();
            let n2 = c2.to_digit(10).unwrap();
            let mut sum = n1 + n2 + carry;

            if sum < 10 {
                carry = 0;
            } else {
                carry = 1;
                sum -= 10;
            }

            result.push(char::from_digit(sum, 10).unwrap());
        }

        // handle the last digit with carry
        if carry == 1 {
            result.push('1');
        } else if result.is_empty() {
            result.push('0')
        }

        // convert result to reverse string and return
        result.chars().rev().collect::<String>()
    }
}

#[test]
fn test() {
    let a = "0".to_string();
    let b = "0".to_string();
    let c = "0".to_string();
    assert_eq!(Solution::add_strings(a, b), c);
}
