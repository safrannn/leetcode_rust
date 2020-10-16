struct Solution;

impl Solution {
    fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k: usize = (k as usize) % nums.len();
        nums.rotate_right(k)
    }
}

#[test]
fn test() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(&mut nums, 3);
    assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
}
