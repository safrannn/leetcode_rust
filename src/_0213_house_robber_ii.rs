struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();
        if n == 0 {
            return 0;
        } else if n == 1 {
            return nums[0];
        }
        let result: i32 = Self::check(&nums, 0, n - 1).max(Self::check(&nums, 1, n));
        result
    }

    fn check(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
        let mut max_prev: i32 = 0;
        let mut max_cur: i32 = 0;

        for i in start..end {
            let temp: i32 = max_cur;
            max_cur = max_cur.max(max_prev + nums[i]);
            max_prev = temp;
        }
        max_cur
    }
}
#[test]
fn test() {
    assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob(vec![0]), 0);
}
