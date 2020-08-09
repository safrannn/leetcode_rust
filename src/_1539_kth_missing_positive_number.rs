struct Solution;

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut arr = arr;
        let mut k = k;
        let mut h_dummy: i32 = 1;
        let mut index: usize = 0;
        while k > 0 {
            if index < arr.len() && h_dummy == arr[index] {
                index += 1;
            } else {
                k -= 1;
            }
            h_dummy += 1;
        }
        h_dummy - 1
    }
}
#[test]
fn test() {
    assert_eq!(Solution::find_kth_positive(vec![2, 3, 4, 7, 11], 5), 9);
    assert_eq!(Solution::find_kth_positive(vec![1, 2, 3, 4], 2), 6);
}
