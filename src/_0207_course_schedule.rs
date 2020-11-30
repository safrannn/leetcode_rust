struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;

        let mut indegrees: Vec<i32> = vec![0; n];
        let mut edges: Vec<Vec<usize>> = vec![vec![]; n];

        for preq in prerequisites {
            let u = preq[0] as usize;
            let v = preq[1] as usize;
            indegrees[v] += 1;
            edges[u].push(v);
        }

        let mut queue: Vec<usize> = vec![];
        for i in 0..n {
            if indegrees[i] == 0 {
                queue.push(i);
            }
        }

        let mut result: Vec<usize> = vec![];
        while let Some(u) = queue.pop() {
            result.push(u);
            while let Some(v) = edges[u].pop() {
                indegrees[v] -= 1;
                if indegrees[v] == 0 {
                    queue.push(v);
                }
            }
        }
        result.len() == n
    }
}

#[test]
fn test() {
    assert_eq!(Solution::can_finish(2, vec_vec_i32![[1, 0]]), true);
    assert_eq!(Solution::can_finish(2, vec_vec_i32![[1, 0], [0, 1]]), false);
}
