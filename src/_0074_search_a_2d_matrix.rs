// struct Solution;

// impl Solution {
//     pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
//         let m: usize = matrix.len();
//         if m == 0 {
//             return false;
//         }
//         let n: usize = matrix[0].len();
//         if n == 0 {
//             return false;
//         }
//         let mut row: usize = 0;
//         let mut col: usize = n - 1;
//         while row < m && col < std::usize::MAX {
//             if matrix[row][col] == target {
//                 return true;
//             } else if matrix[row][col] < target {
//                 row += 1;
//             } else {
//                 col -= 1;
//             }
//         }
//         false
//     }
// }

// #[test]
// fn test() {
//     assert_eq!(
//         Solution::search_matrix(
//             vec_vec_i32![[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 50]],
//             13
//         ),
//         false
//     );
//     assert_eq!(
//         Solution::search_matrix(
//             vec_vec_i32![[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 50]],
//             3
//         ),
//         true
//     );
//     assert_eq!(Solution::search_matrix(vec_vec_i32![], 0), false);
// }
