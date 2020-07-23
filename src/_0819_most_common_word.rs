use std::collections::HashMap;
use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let paragraph = paragraph
            .replace(",", " ")
            .replace(".", " ")
            .replace("!", " ")
            .replace("?", " ")
            .replace("'", " ")
            .replace(";", " ");
        let words: Vec<&str> = paragraph.split_ascii_whitespace().collect();
        let mut h_map_words: HashMap<String, u32> = HashMap::new();
        let h_map_banned: HashSet<String> = banned.into_iter().collect();

        for word in words {
            let w = word.to_string().to_lowercase();
            if !h_map_banned.contains(&w) {
                let count = h_map_words.entry(w).or_insert(0);
                *count += 1;
            }
        }

        let mut max_word: String = "".to_string();
        let mut max_count: u32 = 0;
        for (v, i) in h_map_words {
            if max_count < i {
                max_count = i;
                max_word = v.to_string();
            }
        }
        max_word
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::most_common_word(
            "Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
            vec_string!["hit"]
        ),
        "ball".to_string()
    );
}
