struct Solution;
impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let length = arr.len();
        let offset = length / 4;
        for i in 0..length - offset {
            if arr[i] == arr[i + offset] {
                return arr[i];
            }
        }
        0
    }
}

#[test]
fn test() {
    let arr1 = vec![1, 2, 2, 6, 6, 6, 6, 7, 10];
    assert_eq!(Solution::find_special_integer(arr1), 6);
}
