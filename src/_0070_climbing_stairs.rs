struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        match n {
            0 => 1,
            1 => 1,
            _ => {
                let mut step = vec![0 as i32; n as usize + 1];
                step[0] = 1;
                step[1] = 1;
                for i in 2..=n {
                    step[i as usize] = step[i as usize - 1] + step[i as usize - 2];
                }
                return step[n as usize];
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::climb_stairs(2), 2);
    assert_eq!(Solution::climb_stairs(3), 3);
}
