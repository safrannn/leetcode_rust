struct Solution;
impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        if points.len() < 1 || k == 0 {
            return vec![];
        }
        let mut points = points;
        let pivot: Vec<i32> = points.pop().unwrap();
        let (larger, mut smaller): (Vec<Vec<_>>, Vec<Vec<_>>) =
            points.into_iter().partition(|point| {
                point[0] * point[0] + point[1] * point[1]
                    > pivot[0] * pivot[0] + pivot[1] * pivot[1]
            });

        let smaller_len: i32 = smaller.len() as i32;
        if smaller_len == k {
            return smaller;
        } else if smaller_len > k {
            return Self::k_closest(smaller, k);
        } else {
            smaller.push(pivot);
            let mut to_append: Vec<Vec<i32>> = Self::k_closest(larger, k - smaller_len - 1);
            smaller.append(&mut to_append);
            return smaller;
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::k_closest(vec_vec_i32![[1, 3], [-2, 2]], 1),
        vec_vec_i32![[-2, 2]]
    );
}
