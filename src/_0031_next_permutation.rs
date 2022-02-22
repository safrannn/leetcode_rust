struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let (mut i, mut switched) = (nums.len() - 1, false);
        while i > 0 && nums[i - 1] >= nums[i] {
            i -= 1;
        }
        if i != 0 {
            let (pivot, mut smallest_larger_index) = (nums[i - 1], i);
            for j in i..nums.len() {
                if nums[j] > pivot && nums[j] <= nums[smallest_larger_index] {
                    smallest_larger_index = j;
                }
            }
            nums.swap(i - 1, smallest_larger_index);
            nums[i..].sort()
        } else {
            nums.sort()
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::next_permutation(vec![1, 2, 3]), vec![1, 3, 2]);
    assert_eq!(Solution::next_permutation(vec![3, 2, 1]), vec![1, 2, 3]);
    assert_eq!(Solution::next_permutation(vec![1, 1, 5]), vec![1, 5, 1]);
}
