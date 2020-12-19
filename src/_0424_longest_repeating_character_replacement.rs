struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let n: usize = s.len();
        let s = s.as_bytes();
        let mut left: usize = 0;
        let mut right: usize = 0;
        let mut unique: usize = 0;
        let mut max_len: usize = 0;
        let mut counts: Vec<usize> = vec![0; 26];

        while right < n {
            counts[s[right] as usize - 'A' as usize] += 1;
            unique = unique.max(counts[s[right] as usize - 'A' as usize]);
            let replacement = right - left + 1 - unique;
            if replacement > k as usize {
                counts[s[left] as usize - 'A' as usize] -= 1;
                left += 1;
            } else {
                max_len = max_len.max(right - left + 1);
            }
            right += 1;
        }

        return max_len as i32;
    }
}

#[test]
fn test() {
    assert_eq!(Solution::character_replacement("ABAB".to_string(), 2), 4);
    assert_eq!(Solution::character_replacement("AABABBA".to_string(), 1), 4);
}
