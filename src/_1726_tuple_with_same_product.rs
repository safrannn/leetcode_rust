use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut result = 0;
        let mut products: HashMap<i32, i32> = HashMap::new();
        for i in 0..n - 1 {
            for j in i + 1..n {
                *products.entry(nums[i] * nums[j]).or_default() += 1;
            }
        }

        for (_, v) in products {
            result += v * (v - 1) / 2;
        }
        result * 8
    }
}

#[test]
fn test() {
    assert_eq!(Solution::tuple_same_product(vec![2, 3, 4, 6]), 8);
    assert_eq!(Solution::tuple_same_product(vec![1, 2, 4, 5, 10]), 16);
    assert_eq!(Solution::tuple_same_product(vec![2, 3, 4, 6, 8, 12]), 40);
    assert_eq!(Solution::tuple_same_product(vec![2, 3, 5, 7]), 0);
}
