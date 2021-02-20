struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut strings: Vec<String> = nums.iter().map(|n| n.to_string()).collect();
        strings.sort_unstable_by(|a, b| (b.clone() + &a.clone()).cmp(&(a.clone() + &b.clone())));
        let mut result = "".to_string();
        for s in strings {
            result += &s.clone();
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::largest_number(vec![10, 2]), "210".to_string());
    assert_eq!(
        Solution::largest_number(vec![3, 30, 34, 5, 9]),
        "9534330".to_string()
    );
    assert_eq!(Solution::largest_number(vec![1]), "1".to_string());
    assert_eq!(Solution::largest_number(vec![10]), "10".to_string());
    assert_eq!(Solution::largest_number(vec![0]), "0".to_string());
}
