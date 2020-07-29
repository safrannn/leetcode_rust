struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut h_left: usize = 0;
        let mut h_right: usize = height.len() - 1;
        let mut max_area: i32 = 0;

        while h_left < h_right {
            let h_area: i32 = height[h_left].min(height[h_right]) * (h_right - h_left) as i32;
            max_area = max_area.max(h_area);
            if height[h_left] < height[h_right] {
                h_left += 1;
            } else {
                h_right -= 1;
            }
        }
        max_area
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
}
