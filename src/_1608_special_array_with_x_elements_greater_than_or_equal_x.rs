struct Solution;
impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        // let mut nums = nums;
        // nums.sort_unstable();

        for i in 0..=nums.len() as i32 {
            let mut count: i32 = 0;
            for j in 0..nums.len() {
                if nums[j] >= i {
                    count += 1;
                }
            }
            if count == i {
                return count;
            }
        }
        -1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::special_array(vec![3, 5]), 2);
    assert_eq!(Solution::special_array(vec![0, 0]), -1);
    assert_eq!(Solution::special_array(vec![0, 4, 3, 0, 4]), 3);
    assert_eq!(Solution::special_array(vec![3, 6, 7, 7, 0]), -1);
}
