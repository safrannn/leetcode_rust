struct Solution;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut dictionary: Vec<char> = vec!['a'; 256];
        for (i, v) in order.chars().enumerate() {
            dictionary[v as usize] = (i as u8 + 'a' as u8) as char;
        }

        let words: Vec<String> = words
            .into_iter()
            .map(|word| Solution::translate(&dictionary, word))
            .collect();
        let mut words_sort: Vec<String> = words.to_vec();
        words_sort.sort();
        words == words_sort
    }

    fn translate(dictionary: &Vec<char>, word: String) -> String {
        word.chars().map(|c| dictionary[c as usize]).collect()
    }
}

#[test]
fn test() {
    let words: Vec<String> = vec!["hello".to_string(), "leetcode".to_string()];
    let order: String = "hlabcdefgijkmnopqrstuvwxyz".to_string();
    assert_eq!(Solution::is_alien_sorted(words, order), true);
}
