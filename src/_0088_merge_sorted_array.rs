struct Solution;

impl Solution {
    pub fn merge_sorted_array(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m - 1;
        let mut j = n - 1;
        let mut k = m + n - 1;

        while k >= 0 {
            if i >= 0 && (j < 0 || nums1[i as usize] >= nums2[j as usize]) {
                nums1[k as usize] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[k as usize] = nums2[j as usize];
                j -= 1;
            }
            k -= 1;
        }
    }
}

#[test]
fn test() {
    let mut arr1 = vec![1, 2, 3, 0, 0, 0];
    let mut arr2 = vec![2, 5, 6];
    let m: i32 = 3;
    let n: i32 = 3;
    let answer = vec![1, 2, 2, 3, 5, 6];
    Solution::merge_sorted_array(&mut arr1, m, &mut arr2, n);
    assert_eq!(arr1, answer);
}
