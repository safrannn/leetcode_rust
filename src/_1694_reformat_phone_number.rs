struct Solution;

impl Solution {
    pub fn reformat_number(number: String) -> String {
        let nums: Vec<char> = number.chars().filter(|&c| c >= '0' && c <= '9').collect();
        let n = nums.len();
        let tail = n % 3;

        let mut result = "".to_string();
        for i in 0..n {
            if (tail == 1 && i == n - 2) || (i % 3 == 0 && i != 0) {
                result.push('-');
            }
            result.push(nums[i]);
            if tail == 1 && i == n - 2 {
                result.push(nums[i + 1]);
                break;
            }
        }
        result
    }
}
#[test]
fn test() {
    assert_eq!(
        Solution::reformat_number("1-23-45 6".to_string()),
        "123-456".to_string()
    );
    assert_eq!(
        Solution::reformat_number("123 4-567".to_string()),
        "123-45-67".to_string()
    );
    assert_eq!(
        Solution::reformat_number("123 4-5678".to_string()),
        "123-456-78".to_string()
    );
    assert_eq!(
        Solution::reformat_number("12".to_string()),
        "12".to_string()
    );
    assert_eq!(
        Solution::reformat_number("--17-5 229 35-39475 ".to_string()),
        "175-229-353-94-75".to_string()
    );
}
