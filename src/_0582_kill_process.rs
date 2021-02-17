struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn kill_process(pid: Vec<i32>, ppid: Vec<i32>, kill: i32) -> Vec<i32> {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        let n = pid.len();
        for i in 0..n {
            map.entry(ppid[i]).or_default().push(pid[i]);
        }

        let mut result = vec![kill];
        let mut queue: Vec<i32> = vec![kill];
        while !queue.is_empty() {
            let mut temp: Vec<i32> = vec![];
            for v in queue {
                if let Some(children) = map.get(&v) {
                    for child in children {
                        result.push(child.clone());
                        temp.push(child.clone());
                    }
                }
            }
            queue = temp;
        }

        return result;
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::kill_process(vec![1, 3, 10, 5, 11], vec![3, 0, 5, 3, 5], 5),
        vec![5, 10, 11]
    );
}
