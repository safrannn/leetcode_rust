use std::collections::{HashMap, VecDeque};

struct Solution;
impl Solution {
    pub fn minimum_semesters(n: i32, relations: Vec<Vec<i32>>) -> i32 {
        let mut adjacents: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut indegrees: Vec<i32> = vec![0; n as usize + 1];
        let mut queue: VecDeque<i32> = VecDeque::new();
        let mut answer = 0;

        for v in relations {
            adjacents.entry(v[0]).or_default().push(v[1]);
            indegrees[v[1] as usize] += 1;
        }

        for i in 1..=n as usize {
            if indegrees[i] == 0 {
                queue.push_back(i as i32);
            }
        }

        while !queue.is_empty() {
            answer += 1;
            let m = queue.len();
            for _ in 0..m {
                let u = queue.pop_front().unwrap();
                if let Some(adjs) = adjacents.get(&u) {
                    for &w in adjs.iter() {
                        indegrees[w as usize] -= 1;
                        if indegrees[w as usize] == 0 {
                            queue.push_back(w);
                        }
                    }
                }
            }
        }

        for v in indegrees {
            if v > 0 {
                return -1;
            }
        }
        answer
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::minimum_semesters(3, vec_vec_i32![[1, 3], [2, 3]]),
        2
    );
    assert_eq!(
        Solution::minimum_semesters(3, vec_vec_i32![[1, 2], [2, 3], [3, 1]]),
        -1
    );
}
