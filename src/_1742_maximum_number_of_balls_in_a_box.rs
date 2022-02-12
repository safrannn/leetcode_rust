struct Solution;

impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut occur = vec![0; 46];
        for i in low_limit..high_limit + 1 {
            let (mut sum, mut i) = (0, i);
            while i > 0 {
                sum += i % 10;
                i /= 10;
            }
            occur[sum as usize] += 1;
        }
        occur.into_iter().max().unwrap()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_balls(1, 10), 2);
    assert_eq!(Solution::count_balls(5, 15), 2);
    assert_eq!(Solution::count_balls(19, 28), 2);
}
