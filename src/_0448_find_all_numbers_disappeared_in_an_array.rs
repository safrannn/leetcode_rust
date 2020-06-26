struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        for i in 0..nums.len() {
            let index: usize = nums[i].abs() as usize - 1;
            if nums[index] > 0 {
                nums[index] *= -1;
            }
        }
        let mut result = vec![];
        for (i, &v) in nums.iter().enumerate() {
            if v > 0 {
                result.push(i as i32 + 1);
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
        vec![5, 6]
    );
}
