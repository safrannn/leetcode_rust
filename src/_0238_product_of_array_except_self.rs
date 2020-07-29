struct Solution;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let length: usize = nums.len();
        let mut answer: Vec<i32> = vec![1; length];

        let mut h_product: i32 = 1;
        for i in 0..length {
            answer[i] = h_product;
            h_product *= nums[i]
        }
        h_product = 1;
        for i in (0..length).rev() {
            answer[i] *= h_product;
            h_product *= nums[i];
        }
        answer
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::product_except_self(vec![1, 2, 3, 4]),
        vec![24, 12, 8, 6]
    );
}
