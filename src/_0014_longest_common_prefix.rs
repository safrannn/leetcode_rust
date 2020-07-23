struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }

        let h_length_shortest: usize = strs.iter().map(|s| s.len()).min().unwrap_or(0);
        let mut index: usize = 0;
        'outer: for _i in 0..h_length_shortest {
            let h_character: u8 = strs[0].as_bytes()[index];
            for j in 1..strs.len() {
                if strs[j].as_bytes()[index] != h_character {
                    break 'outer;
                }
            }
            index += 1;
        }
        strs[0][0..index].to_string()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::longest_common_prefix(vec_string!["flower", "flow", "flight"]),
        "fl".to_string()
    );
}
