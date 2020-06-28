struct Solution;

impl Solution {
    pub fn shortest_distance(words: Vec<String>, word1: String, word2: String) -> i32 {
        let mut pos_1: i32 = words.len() as i32;
        let mut pos_2: i32 = words.len() as i32;
        let mut diff: i32 = words.len() as i32;

        for (i, word) in words.iter().enumerate() {
            if word.eq(&word1) {
                pos_1 = i as i32;
                diff = (pos_2 - pos_1).abs().min(diff);
            } else if word.eq(&word2) {
                pos_2 = i as i32;
                diff = (pos_2 - pos_1).abs().min(diff);
            }
        }
        diff
    }
}

#[test]
fn test() {
    let words: Vec<String> = vec_string!["practice", "makes", "perfect", "coding", "makes"];
    let word1 = "coding".to_string();
    let word2 = "practice".to_string();
    assert_eq!(Solution::shortest_distance(words, word1, word2), 3);
    let words: Vec<String> = vec_string!["practice", "makes", "perfect", "coding", "makes"];
    let word1 = "makes".to_string();
    let word2 = "coding".to_string();
    assert_eq!(Solution::shortest_distance(words, word1, word2), 1);
}
