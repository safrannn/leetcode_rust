struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let mut start: usize = 0;
        let mut total_gas: i32 = 0;
        let mut cur_tank: i32 = 0;

        for i in 0..n {
            total_gas += gas[i] - cost[i];
            cur_tank += gas[i] - cost[i];
            if cur_tank < 0 {
                start = i + 1;
                cur_tank = 0;
            }
        }

        if total_gas < 0 {
            -1
        } else {
            start as i32
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
        3
    );
    assert_eq!(
        Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]),
        -1
    );
}
