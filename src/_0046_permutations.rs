struct Solution;

impl Solution {
    fn permute_(nums: &mut Vec<i32>, start: usize, end: usize, result: &mut Vec<Vec<i32>>) {
        if start == end {
            result.push(nums.to_vec());
            return;
        } else {
            for i in start..end {
                nums.swap(start, i);
                Self::permute_(nums, start + 1, end, result);
                nums.swap(start, i);
            }
        }
    }
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let end: usize = nums.len();
        Self::permute_(&mut nums, 0, end, &mut result);
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::permute(vec![1, 2, 3]),
        vec![
            [1, 2, 3],
            [1, 3, 2],
            [2, 1, 3],
            [2, 3, 1],
            [3, 2, 1],
            [3, 1, 2]
        ]
    );
}
