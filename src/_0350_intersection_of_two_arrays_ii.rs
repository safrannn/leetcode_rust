use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map_1: HashMap<i32, i32> = HashMap::new();
        nums1.iter().for_each(|&x| {
            *map_1.entry(x).or_default() += 1;
        });
        let mut map_2: HashMap<i32, i32> = HashMap::new();
        nums2.iter().for_each(|&x| {
            *map_2.entry(x).or_default() += 1;
        });

        let mut result: Vec<i32> = vec![];
        for (n1, o1) in map_1 {
            if let Some(&o2) = map_2.get(&n1.clone()) {
                for i in 0..o1.min(o2) {
                    result.push(n1.clone());
                }
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]),
        vec![2, 2]
    );
}
