struct Solution;
use crate::util::*;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];

        let n: usize = nums.len();
        if n < 3 {
            return result;
        }
        nums.sort_unstable();

        for i in 0..n - 2 {
            if nums[i] > 0 {
                break;
            }
            if i > 0 && nums[i - 1] == nums[i] {
                continue;
            }

            let target: i32 = -nums[i];
            let mut left: usize = i + 1;
            let mut right: usize = n - 1;
            while left < right {
                if nums[left] + nums[right] < target {
                    left += 1;
                } else if nums[left] + nums[right] > target {
                    right -= 1;
                } else {
                    result.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;
                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                }
            }
        }
        result
    }
}

#[test]
fn test() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let res: Vec<Vec<i32>> = vec_vec_i32![[-1, -1, 2], [-1, 0, 1]];
    assert_eq!(Solution::three_sum(nums), res);
    let nums = vec![-2, 0, 1, 1, 2];
    let res: Vec<Vec<i32>> = vec_vec_i32![[-2, 0, 2], [-2, 1, 1]];
    assert_eq!(Solution::three_sum(nums), res);
}
