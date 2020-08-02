struct Solution;

impl Solution {
    pub fn max_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut sum_1: i64 = 0;
        let mut sum_2: i64 = 0;
        let mut pointer_1: usize = 0;
        let mut pointer_2: usize = 0;
        let mut result: i64 = 0;

        while pointer_1 < nums1.len() && pointer_2 < nums2.len() {
            // println!("{},{}",sum_1,sum_2);
            let v_1 = nums1[pointer_1] as i64;
            let v_2 = nums2[pointer_2] as i64;
            if v_1 < v_2 {
                // println!("1");
                sum_1 += v_1;
                if pointer_1 < nums1.len() {
                    pointer_1 += 1;
                } else {
                    break;
                }
            } else if v_1 > v_2 {
                println!("2");
                sum_2 += v_2;
                if pointer_2 < nums2.len() {
                    pointer_2 += 1;
                } else {
                    break;
                }
            } else {
                println!("3");
                pointer_1 += 1;
                pointer_2 += 1;
                result += sum_1.max(sum_2) + v_1;
                sum_1 = 0;
                sum_2 = 0;
            }
        }
        while pointer_1 < nums1.len() {
            sum_1 += nums1[pointer_1] as i64;
            pointer_1 += 1;
        }
        while pointer_2 < nums2.len() {
            sum_2 += nums2[pointer_2] as i64;
            pointer_2 += 1;
        }
        let result_new: i32 = ((result + sum_1.max(sum_2)) % (1000000000 + 7)) as i32;
        result_new
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::max_sum(vec![2, 4, 5, 8, 10], vec![4, 6, 8, 9]),
        30
    );
    assert_eq!(Solution::max_sum(vec![1, 3, 5, 7, 9], vec![3, 5, 100]), 109);
    assert_eq!(
        Solution::max_sum(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]),
        40
    );
    assert_eq!(
        Solution::max_sum(vec![1, 4, 5, 8, 9, 11, 19], vec![2, 3, 4, 11, 12]),
        61
    );
}
