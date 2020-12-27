use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;
impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let n = apples.len();
        let mut count = 0;
        let mut heap = BinaryHeap::new();
        let mut last_day = 0;
        for i in 0..n {
            last_day = last_day.max(i + days[i] as usize);
        }

        for i in 0..last_day {
            if i < n && apples[i] != 0 {
                heap.push(Reverse((i + days[i] as usize, apples[i])));
            }

            while let Some(Reverse(today)) = heap.pop() {
                if today.1 > 0 && today.0 > i {
                    count += 1;
                    heap.push(Reverse((today.0, today.1 - 1)));
                    break;
                }
            }
        }
        count
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::eaten_apples(vec![1, 2, 3, 5, 2], vec![3, 2, 1, 4, 2]),
        7
    );
    assert_eq!(
        Solution::eaten_apples(vec![3, 0, 0, 0, 0, 2], vec![3, 0, 0, 0, 0, 2]),
        5
    );
}
