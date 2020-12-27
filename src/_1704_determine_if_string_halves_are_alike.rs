struct Solution;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let mut count: i32 = 0;
        let s: Vec<char> = s.chars().collect();
        for (i, v) in s.iter().enumerate() {
            match v {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                    if i < s.len() / 2 {
                        count += 1;
                    } else {
                        count -= 1;
                    }
                }
                _ => {}
            }
        }
        return count == 0;
    }
}

#[test]
fn test() {
    assert_eq!(Solution::halves_are_alike("book".to_string()), true);
    assert_eq!(Solution::halves_are_alike("textbook".to_string()), false);
    assert_eq!(
        Solution::halves_are_alike("MerryChristmas".to_string()),
        false
    );
    assert_eq!(Solution::halves_are_alike("AbCdEfGh".to_string()), true);
}
