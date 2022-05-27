struct Solution;

impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        box_types.sort_by_key(|x| -x[1]);
        box_types
            .into_iter()
            .fold((truck_size, 0), |(unit_left, result), box_info| {
                (
                    (unit_left - box_info[0]).max(0),
                    result + box_info[1] * unit_left.min(box_info[0]),
                )
            })
            .1
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::maximum_units(vec_vec_i32![[1, 3], [2, 2], [3, 1]], 4),
        8
    );
    assert_eq!(
        Solution::maximum_units(vec_vec_i32![[5, 10], [2, 5], [4, 7], [3, 9]], 10),
        91
    );
}
