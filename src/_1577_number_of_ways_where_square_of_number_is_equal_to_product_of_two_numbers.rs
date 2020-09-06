struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn num_triplets(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        nums1.sort_unstable();
        nums2.sort_unstable();
        let result_1 = Self::find(nums1.clone(), nums2.clone());
        dbg!(&result_1);
        let result_2 = Self::find(nums2.clone(), nums1.clone());
        dbg!(&result_2);
        result_1 + result_2
    }

    fn find(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut count: i32 = 0;

        let mut target: HashMap<i64, i32> = HashMap::new();
        for i in 0..nums1.len() {
            *target.entry(nums1[i] as i64 * nums1[i] as i64).or_insert(0) += 1;
        }

        let mut new_nums2: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums2.len() {
            *new_nums2.entry(nums2[i]).or_insert(0) += 1;
        }
        let helper: Vec<(i32, i32)> = new_nums2
            .iter()
            .map(|(&k, &v)| (k as i32, v as i32))
            .collect();

        for i in 0..helper.len() {
            for j in i..helper.len() {
                let v1 = helper[i].0 as i64;
                let v2 = helper[j].0 as i64;
                let v = v1 * v2;
                if let Some(&ocur) = target.get(&v) {
                    if v1 == v2 {
                        count += (helper[i].1 * (helper[i].1 - 1)) * ocur / 2;
                    } else {
                        count += (helper[i].1 * helper[j].1) * ocur;
                    }
                }
            }
        }
        count
    }
}
#[test]
fn test() {
    assert_eq!(Solution::num_triplets(vec![7, 4], vec![5, 2, 8, 9]), 1);
    assert_eq!(Solution::num_triplets(vec![1, 1], vec![1, 1, 1]), 9);
    assert_eq!(
        Solution::num_triplets(vec![7, 7, 8, 3], vec![1, 2, 9, 7]),
        2
    );
    assert_eq!(
        Solution::num_triplets(vec![4, 7, 9, 11, 23], vec![3, 5, 1024, 12, 18]),
        0
    );
    assert_eq!(
        Solution::num_triplets(vec![3, 1, 2, 2], vec![1, 3, 4, 4]),
        4
    );
}
