struct Solution;

impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let mut max = -1;
        for i in (0..arr.len()).rev() {
            let t = max;
            max = i32::max(arr[i], max);
            arr[i] = t;
        }
        arr
    }
}

#[test]
fn test() {
    let arr1 = vec![17, 18, 5, 4, 6, 1];
    let arr2 = vec![18, 6, 6, 6, 1, -1];
    assert_eq!(Solution::replace_elements(arr1), arr2);
}
