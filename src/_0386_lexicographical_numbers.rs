struct Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut result = vec![];
        Self::dfs(0, n, &mut result);
        result
    }

    fn dfs(current: i32, n: i32, result: &mut Vec<i32>) {
        if current > n {
            return;
        }
        for i in 0..10 {
            let temp = current * 10 + i;
            if temp != 0 && temp <= n {
                result.push(temp);
                Self::dfs(temp, n, result);
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::lexical_order(13),
        vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
    );
}
