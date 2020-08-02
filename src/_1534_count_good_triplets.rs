struct Solution;

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let n: usize = arr.len();
        let mut result: i32 = 0;
        for i in 0..n {
            for j in i + 1..n {
                if (arr[i] - arr[j]).abs() > a {
                    continue;
                }
                for k in j + 1..n {
                    if (arr[j] - arr[k]).abs() > b || (arr[i] - arr[k]).abs() > c {
                        continue;
                    } else {
                        result += 1;
                    }
                }
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3),
        4
    );
    assert_eq!(
        Solution::count_good_triplets(vec![1, 1, 2, 2, 3], 0, 0, 1),
        0
    )
}
