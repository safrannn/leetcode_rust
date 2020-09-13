struct Solution;

impl Solution {
    pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut pairs_: Vec<i32> = vec![-1; n];
        for i in 0..pairs.len() {
            pairs_[pairs[i][0] as usize] = pairs[i][1];
            pairs_[pairs[i][1] as usize] = pairs[i][0];
        }

        let mut result = vec![false; n];
        for pair in pairs {
            let left = pair[0];
            let right = pair[1];

            if !result[left as usize] {
                let left_prefs = &preferences[left as usize];
                for i in 0..left_prefs.len() {
                    let u = left_prefs[i];
                    if u == right {
                        break;
                    }
                    let v = pairs_[u as usize];

                    for j in 0..n - 1 {
                        if preferences[u as usize][j] == left {
                            result[left as usize] = true;
                            result[u as usize] = true;
                            break;
                        } else if preferences[u as usize][j] == v {
                            break;
                        }
                    }
                }
            }

            if !result[right as usize] {
                let right_prefs = &preferences[right as usize];
                for i in 0..right_prefs.len() {
                    let u = right_prefs[i];
                    if u == left {
                        break;
                    }
                    let v = pairs_[u as usize];

                    for j in 0..n - 1 {
                        if preferences[u as usize][j] == right {
                            result[right as usize] = true;
                            result[u as usize] = true;
                            break;
                        } else if preferences[u as usize][j] == v {
                            break;
                        }
                    }
                }
            }
        }
        let mut answer: i32 = 0;
        for v in result {
            if v {
                answer += 1;
            }
        }
        answer
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::unhappy_friends(
            4,
            vec_vec_i32![[1, 2, 3], [3, 2, 0], [3, 1, 0], [1, 2, 0]],
            vec_vec_i32![[0, 1], [2, 3]]
        ),
        2
    );
    assert_eq!(
        Solution::unhappy_friends(2, vec_vec_i32![[1], [0]], vec_vec_i32![[1, 0]]),
        0
    );
    assert_eq!(
        Solution::unhappy_friends(
            4,
            vec_vec_i32![[1, 3, 2], [2, 3, 0], [1, 3, 0], [0, 2, 1]],
            vec_vec_i32![[1, 3], [0, 2]]
        ),
        4
    );
}
