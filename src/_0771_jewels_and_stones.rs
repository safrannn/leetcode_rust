struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let mut jewels = vec![false; 58];
        for v in j.chars() {
            jewels[(v as u8 - 'A' as u8) as usize] = true;
        }

        let mut result: i32 = 0;
        for v in s.chars() {
            if jewels[(v as u8 - 'A' as u8) as usize] {
                result += 1;
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()),
        3
    );
    assert_eq!(
        Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string()),
        0
    );
}
