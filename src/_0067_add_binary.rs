struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a_ = u128::from_str_radix(&a, 2).unwrap_or(0);
        let b_ = u128::from_str_radix(&b, 2).unwrap_or(0);
        format!("{:b}", a_ + b_)
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::add_binary("11".to_string(), "1".to_string()),
        "100".to_string()
    );

    assert_eq!(
        Solution::add_binary("1010".to_string(), "1011".to_string()),
        "10101".to_string()
    );
}
