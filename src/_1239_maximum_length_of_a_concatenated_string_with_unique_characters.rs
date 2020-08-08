struct Solution;

impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let arr: Vec<u32> = arr
            .into_iter()
            .filter_map(|s| {
                let mut word_bit: u32 = 0;
                for b in s.bytes() {
                    let b_: u32 = 1 << (b - b'a');
                    if b_ & word_bit != 0 {
                        return None;
                    } else {
                        word_bit |= b_;
                    }
                }
                Some(word_bit)
            })
            .collect();

        let mut result: u32 = 0;
        Self::dfs(&arr, 0, &mut result, 0);
        result as i32
    }

    fn dfs(arr: &Vec<u32>, index: usize, maximum: &mut u32, current_concat: u32) {
        if index == arr.len() {
            *maximum = (*maximum).max(current_concat.count_ones());
        } else {
            if arr[index] & current_concat == 0 {
                Self::dfs(arr, index + 1, maximum, current_concat | arr[index]);
            }
            Self::dfs(arr, index + 1, maximum, current_concat);
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_length(vec_string!["un", "iq", "ue"]), 4);
    assert_eq!(
        Solution::max_length(vec_string!["cha", "r", "act", "ers"]),
        6
    );
    assert_eq!(
        Solution::max_length(vec_string!["abcdefghijklmnopqrstuvwxyz"]),
        26
    );
}
