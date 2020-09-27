struct Solution;
impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut depth: i32 = 0;
        for log in logs {
            if log == "./" {
                continue;
            } else if log == "../" {
                if depth > 0 {
                    depth -= 1;
                }
            } else {
                depth += 1;
            }
        }
        depth
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::min_operations(vec_string!["d1/", "d2/", "../", "d21/", "./"]),
        2
    );
    assert_eq!(
        Solution::min_operations(vec_string!["d1/", "d2/", "./", "d3/", "../", "d31/"]),
        3
    );
    assert_eq!(
        Solution::min_operations(vec_string!["d1/", "../", "../", "../"]),
        0
    );
}
