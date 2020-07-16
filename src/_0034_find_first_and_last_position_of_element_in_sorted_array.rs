struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len();
        match nums.binary_search(&target) {
            Ok(i) => {
                let mut left = i;
                while left > 0 && nums[left - 1] == target {
                    left -= 1
                }
                let mut right = i;
                while right < n - 1 && nums[right + 1] == target {
                    right += 1;
                }
                vec![left as i32, right as i32]
            }
            Err(_) => vec![-1, -1],
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
        vec![3, 4]
    );
    assert_eq!(
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
        vec![-1, -1]
    );
}
