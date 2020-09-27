struct Solution;

impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let words: Vec<&str> = text.split_whitespace().collect();
        let word_count = words.len() as i32;
        if word_count == 0 {
            return text;
        }
        let mut white_count = text.matches(" ").count() as i32;
        if white_count == 0 {
            return text;
        }
        let mut result = "".to_string();
        let white_each;
        if word_count != 1 {
            white_each = white_count / (word_count - 1);
        } else {
            white_each = white_count;
        }
        for w in words {
            result += w;
            for _i in 0..white_each {
                if white_count > 0 {
                    result += " ";
                    white_count -= 1;
                }
            }
        }

        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::reorder_spaces("  this   is  a sentence ".to_string()),
        "this   is   a   sentence".to_string()
    );
    assert_eq!(
        Solution::reorder_spaces(" practice   makes   perfect".to_string()),
        "practice   makes   perfect ".to_string()
    );
    assert_eq!(
        Solution::reorder_spaces("hello   world".to_string()),
        "hello   world".to_string()
    );
    assert_eq!(
        Solution::reorder_spaces("  walks  udp package   into  bar a".to_string()),
        "walks  udp  package  into  bar  a ".to_string()
    );
    assert_eq!(Solution::reorder_spaces("a".to_string()), "a".to_string());
}
