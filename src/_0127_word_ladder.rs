struct Solution;
use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut level: i32 = 0;
        let n: usize = begin_word.len();
        let begin_list: Vec<Vec<u8>> = vec![begin_word.as_bytes().to_vec()];
        let end_list: Vec<Vec<u8>> = vec![end_word.as_bytes().to_vec()];
        let mut begin_set: HashSet<Vec<u8>> = HashSet::from_iter(begin_list);
        let mut end_set: HashSet<Vec<u8>> = HashSet::from_iter(end_list);
        let mut unused_set: HashSet<Vec<u8>> =
            HashSet::from_iter(word_list.into_iter().map(|s| s.as_bytes().to_vec()));
        if !unused_set.contains(&end_word.as_bytes().to_vec()) {
            return level;
        } else {
            level = 1;
        }
        while !begin_set.is_empty() {
            level += 1;
            let mut temp_set: HashSet<Vec<u8>> = HashSet::new();
            for word in begin_set.iter() {
                let mut w: Vec<u8> = word.to_vec();
                for i in 0..n {
                    let c = w[i];
                    for offset in 0..26 {
                        w[i] = b'a' + offset;
                        if end_set.contains(&w) {
                            return level;
                        }
                        if unused_set.contains(&w) {
                            unused_set.remove(&w);
                            temp_set.insert(w.clone());
                        }
                    }
                    w[i] = c;
                }
            }
            std::mem::swap(&mut begin_set, &mut temp_set);
            // *begin_set = temp_set;
            if begin_set.len() > end_set.len() {
                std::mem::swap(&mut begin_set, &mut end_set);
            }
        }
        0
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::ladder_length(
            "hit".to_string(),
            "cog".to_string(),
            vec_string!["hot", "dot", "dog", "lot", "log", "cog"]
        ),
        5
    );

    assert_eq!(
        Solution::ladder_length(
            "hit".to_string(),
            "cog".to_string(),
            vec_string!["hot", "dot", "dog", "lot", "log"]
        ),
        0
    );
}
