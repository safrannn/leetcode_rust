struct Solution;

impl Solution {
    pub fn sort_array_by_parity(mut a: Vec<i32>) -> Vec<i32> {
        let mut even: usize = 0;
        let mut odd: usize = a.len() - 1;
        while even < odd {
            while a[even] % 2 == 0 && even < odd {
                even += 1;
            }
            while a[odd] % 2 == 1 && even < odd {
                odd -= 1;
            }
            if even < odd {
                a.swap(even, odd);
            }
        }
        a
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::sort_array_by_parity(vec![3, 1, 2, 4]),
        vec![4, 2, 1, 3]
    );
}
