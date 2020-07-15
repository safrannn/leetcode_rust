struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() == 0 {
            return vec![];
        } else if intervals.len() == 1 {
            return intervals;
        }

        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut result: Vec<Vec<i32>> = vec![intervals[0].clone()];

        for i in 1..intervals.len() {
            let mut a = result.pop().unwrap();
            let b = intervals[i].clone();
            if Self::overlap(&a, &b) {
                a[1] = a[1].max(b[1]);
                result.push(a);
            } else {
                result.push(a);
                result.push(b);
            }
        }
        result
    }

    fn overlap(a: &Vec<i32>, b: &Vec<i32>) -> bool {
        if a[1] >= b[0] {
            return true;
        }
        false
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::merge(vec_vec_i32![[1, 3], [2, 6], [8, 10], [15, 18]]),
        vec_vec_i32![[1, 6], [8, 10], [15, 18]]
    );
    assert_eq!(
        Solution::merge(vec_vec_i32![[1, 4], [4, 5]]),
        vec_vec_i32![[1, 5]]
    );
    assert_eq!(
        Solution::merge(vec_vec_i32![[1, 4], [2, 3]]),
        vec_vec_i32![[1, 4]]
    );
}
