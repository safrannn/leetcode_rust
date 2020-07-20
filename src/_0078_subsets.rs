struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut current: Vec<i32> = vec![];
        let n: usize = nums.len();
        Self::dfs(&nums, 0, &n, &mut result, &mut current);
        result
    }

    fn dfs(
        nums: &Vec<i32>,
        i: usize,
        n: &usize,
        result: &mut Vec<Vec<i32>>,
        current: &mut Vec<i32>,
    ) {
        if i == *n {
            result.push(current.to_vec());
        } else {
            Self::dfs(nums, i + 1, n, result, current);
            current.push(nums[i]);
            Self::dfs(nums, i + 1, n, result, current);
            current.pop();
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::subsets(vec![1, 2, 3]),
        vec_vec_i32![[], [3], [2], [2, 3], [1], [1, 3], [1, 2], [1, 2, 3]]
    );
}
