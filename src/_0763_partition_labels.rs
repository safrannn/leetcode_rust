struct Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut h_occur_last: Vec<usize> = vec![0; 26];
        for (i, v) in s.chars().enumerate() {
            h_occur_last[v as usize - 'a' as usize] = i;
        }

        let mut h_anchor: usize = 0;
        let mut h_block_last: usize = 0;
        let mut result: Vec<i32> = vec![];
        for (i, v) in s.chars().enumerate() {
            h_block_last = h_block_last.max(h_occur_last[v as usize - 'a' as usize]);
            if i == h_block_last {
                result.push((i - h_anchor + 1) as i32);
                h_anchor = i + 1;
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::partition_labels("ababcbacadefegdehijhklij".to_string()),
        vec![9, 7, 8]
    );
}
