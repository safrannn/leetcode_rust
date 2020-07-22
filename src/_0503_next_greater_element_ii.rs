struct Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result: Vec<i32> = vec![-1; n];
        let mut h_stack: Vec<usize> = vec![];

        for i in 0..2 * n {
            let i_: usize = i % n;
            while let Some(last) = h_stack.pop() {
                if nums[i_] > nums[last] {
                    result[last] = nums[i_] as i32;
                } else {
                    h_stack.push(last);
                    break;
                }
            }
            h_stack.push(i_);
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::next_greater_elements(vec![1, 2, 1]),
        vec![2, -1, 2]
    );
}
