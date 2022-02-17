struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Self::dfs(&nums, 0, &mut vec![], &mut result);
        result
    }

    fn dfs(nums: &Vec<i32>, i: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if i == nums.len() {
            result.push(current.clone());
        } else {
            current.push(nums[i]);
            Self::dfs(&nums, i + 1, current, result);
            current.pop();
            Self::dfs(&nums, i + 1, current, result);
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
