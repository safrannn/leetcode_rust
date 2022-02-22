struct Solution;

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut stack = vec![];
        let mut k = k as usize;
        for (i, c) in num.chars().enumerate() {
            if k > 0 {
                while k > 0 {
                    if let Some(&last) = stack.last() {
                        if c < last {
                            stack.pop();
                            k -= 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
            stack.push(c);
        }
        let result = stack
            .split_at(stack.len() - k as usize)
            .0
            .into_iter()
            .collect::<String>()
            .trim_start_matches('0')
            .to_string();
        if result == "" {
            "0".to_string()
        } else {
            result
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::remove_kdigits("1432219".to_string(), 3), "1219");
    assert_eq!(Solution::remove_kdigits("10200".to_string(), 1), "200");
    assert_eq!(Solution::remove_kdigits("10".to_string(), 2), "0");
}
