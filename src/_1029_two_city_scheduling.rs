struct Solution;

impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let n: usize = costs.len();
        let mut diff: Vec<i32> = costs.iter().map(|v| v[0] - v[1]).collect();
        diff.sort_unstable();
        let sum_cityb: i32 = costs.iter().map(|v| v[1]).sum();
        let sum_diff: i32 = diff.iter().take(n / 2).sum();
        sum_cityb + sum_diff
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::two_city_sched_cost(vec_vec_i32![[10, 20], [30, 200], [400, 50], [30, 20]]),
        110
    );
}
