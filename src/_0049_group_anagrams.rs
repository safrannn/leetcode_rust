struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut h_map: HashMap<Vec<char>, Vec<String>> = HashMap::new();
        for str in strs {
            let mut str_: Vec<char> = str.chars().collect();
            str_.sort();
            let h_group = h_map.entry(str_).or_insert(Vec::new());
            (*h_group).push(str);
        }

        let mut result: Vec<Vec<String>> = h_map.into_iter().map(|(_k, v)| v).collect();
        result.sort_by_key(|x| x.len());
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::group_anagrams(vec_string!["eat", "tea", "tan", "ate", "nat", "bat"]),
        vec![
            vec_string!["bat"],
            vec_string!["tan", "nat"],
            vec_string!["eat", "tea", "ate"],
        ]
    );
}
