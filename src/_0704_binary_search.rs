struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut i, mut j) = (0, (nums.len() - 1) as i32);
        while i <= j {
            let mid = (i + j) / 2;
            if nums[mid as usize] == target {
                return mid as i32;
            } else if nums[mid as usize] < target {
                i = mid + 1;
            } else {
                j = mid - 1;
            }
        }
        -1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
}
