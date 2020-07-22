struct Solution;

impl Solution {
    pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
        let sum_total: i32 = a.iter().sum();
        if sum_total % 3 != 0 {
            return false;
        }
        let mut count: i32 = 0;
        let mut sum_temp: i32 = 0;
        for v in a.iter() {
            sum_temp += v;
            if sum_temp == sum_total / 3 {
                count += 1;
                sum_temp = 0;
            }
            if count == 3 {
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1]),
        true
    );
    assert_eq!(
        Solution::can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1]),
        false
    );
    assert_eq!(
        Solution::can_three_parts_equal_sum(vec![3, 3, 6, 5, -2, 2, 5, 1, -9, 4]),
        true
    );
}
