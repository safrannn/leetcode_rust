struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        if chars.is_empty() {
            return 0;
        }
        let mut h_count: i32 = 0;
        let mut h_char_current: char = chars[0];
        let mut h_index: usize = 0;
        let h_length: usize = chars.len();

        for j in 0..=h_length {
            if j == h_length || chars[j] != h_char_current {
                chars[h_index] = h_char_current;
                h_index += 1;
                if j != h_length {
                    h_char_current = chars[j];
                }

                if h_count == 1 {
                    continue;
                } else {
                    let h_number_chars = h_count.to_string().chars().collect::<Vec<char>>();
                    for v in h_number_chars.into_iter() {
                        chars[h_index] = v;
                        h_index += 1;
                    }
                    h_count = 1;
                }
            } else {
                h_count += 1;
            }
        }
        h_index as i32
    }
}

#[test]
fn test() {
    let mut input: Vec<char> = [
        "a", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b",
    ]
    .iter()
    .map(|s| s.chars().next().unwrap())
    .collect();
    assert_eq!(Solution::compress(&mut input), 4);
}
