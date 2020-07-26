struct Solution;

impl Solution {
    pub fn sort_array_by_parity_ii(mut a: Vec<i32>) -> Vec<i32> {
        let mut h_even_index: usize = 0;
        let mut h_odd_index: usize = 1;
        let n: usize = a.len();
        while h_even_index < n && h_odd_index < n {
            while h_even_index < n && a[h_even_index] % 2 == 0 {
                h_even_index += 2;
            }
            while h_odd_index < n && a[h_odd_index] % 2 == 1 {
                h_odd_index += 2;
            }

            if h_even_index < n && h_odd_index < n {
                a.swap(h_even_index, h_odd_index);
            }
        }
        a
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::sort_array_by_parity_ii(vec![4, 2, 5, 7]),
        vec![4, 5, 2, 7]
    );
}
