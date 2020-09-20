use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        let mut hash_set: HashSet<String> = HashSet::new();
        let mut result: usize = 0;
        Self::search(0, hash_set, &mut result, &s);
        result as i32
    }

    fn search(start: usize, mut hash_set: HashSet<String>, max: &mut usize, s: &String) {
        if start == s.len() {
            if *max < hash_set.len() {
                *max = hash_set.len();
                println!("{},{:?},", hash_set.len(), hash_set);
            }
            return;
        }

        for i in start + 1..=s.len() {
            let word = &s[start..i];
            if hash_set.contains(word) {
                Self::search(i, hash_set.clone(), max, s);
            } else {
                hash_set.insert(word.to_string());
                Self::search(i, hash_set.clone(), max, s);
                hash_set.remove(word);
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_unique_split("ababccc".to_string()), 5);
    assert_eq!(Solution::max_unique_split("aba".to_string()), 2);
    assert_eq!(Solution::max_unique_split("aa".to_string()), 1);
}
