struct Solution;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Eq, Ord, PartialEq)]
struct FrequentWord {
    word: String,
    occurrences: usize,
}

impl PartialOrd for FrequentWord {
    fn partial_cmp(&self, other: &FrequentWord) -> Option<Ordering> {
        Some(match self.occurrences.cmp(&other.occurrences) {
            Ordering::Equal => other.word.cmp(&self.word),
            x => x,
        })
    }
}

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut map = HashMap::new();
        words.into_iter().for_each(|word| {
            (*map.entry(word).or_insert(0)) += 1;
        });
        let mut heap = BinaryHeap::with_capacity(map.len());
        map.into_iter().for_each(|(word, occurrences)| {
            heap.push(FrequentWord { word, occurrences });
        });
        (0..k).map(|_| heap.pop().unwrap().word).collect()
    }
}

#[test]
fn test() {
    let words: Vec<String> = vec_string!["i", "love", "leetcode", "i", "love", "coding"];
    let res: Vec<String> = vec_string!["i", "love"];
    let k = 2;
    assert_eq!(Solution::top_k_frequent(words, k), res);
    let words: Vec<String> =
        vec_string!["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"];
    let res: Vec<String> = vec_string!["the", "is", "sunny", "day"];
    let k = 4;
    assert_eq!(Solution::top_k_frequent(words, k), res);
}
