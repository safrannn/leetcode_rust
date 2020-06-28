struct Solution;

impl Solution {
    pub fn min_cost(mut costs: Vec<Vec<i32>>) -> i32 {
        if costs.len() == 0 {
            return 0;
        }

        for i in 1..costs.len() {
            costs[i][0] += std::cmp::min(costs[i - 1][1], costs[i - 1][2]);
            costs[i][1] += std::cmp::min(costs[i - 1][0], costs[i - 1][2]);
            costs[i][2] += std::cmp::min(costs[i - 1][0], costs[i - 1][1]);
        }

        *costs[costs.len() - 1].iter().min().unwrap()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::min_cost(vec![vec![17, 2, 17], vec![16, 16, 5], vec![14, 3, 19]]),
        10
    );
}
