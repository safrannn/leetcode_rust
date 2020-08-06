struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let h_map: HashMap<char, Vec<char>> = [
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
            ('0', " "),
        ]
        .iter()
        .map(|(number, letters)| (*number, letters.chars().collect()))
        .collect();

        let mut current_combination: Vec<char> = vec![];
        let current_index: usize = 0;
        let mut result: Vec<String> = vec![];
        let digits: Vec<char> = digits.chars().collect();
        Self::dfs(
            &h_map,
            &digits,
            &mut current_combination,
            current_index,
            &mut result,
        );
        result
    }

    fn dfs(
        h_map: &HashMap<char, Vec<char>>,
        digits: &Vec<char>,
        current_combination: &mut Vec<char>,
        current_index: usize,
        result: &mut Vec<String>,
    ) {
        if current_index == digits.len() {
            result.push(current_combination.iter().collect::<String>());
        } else {
            let current_digit: char = digits[current_index];
            for &c in h_map[&current_digit].iter() {
                current_combination.push(c);
                Self::dfs(
                    h_map,
                    digits,
                    current_combination,
                    current_index + 1,
                    result,
                );
                current_combination.pop();
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::letter_combinations("23".to_string()),
        vec_string!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    )
}
