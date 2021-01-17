struct Solution;

impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut max_square_side = 0;
        let mut max_square_count = 0;

        for v in rectangles {
            let side = v[0].min(v[1]);
            if side > max_square_side {
                max_square_side = side;
                max_square_count = 1;
            } else if side == max_square_side {
                max_square_count += 1;
            }
        }

        max_square_count
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::count_good_rectangles(vec_vec_i32![[5, 8], [3, 9], [5, 12], [16, 5]]),
        3
    );
    assert_eq!(
        Solution::count_good_rectangles(vec_vec_i32![[2, 3], [3, 7], [4, 3], [3, 7]]),
        3
    );
}
