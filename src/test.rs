// use std::collections::HashMap;

// struct Solution;
// // impl Solution {
// //     pub fn number_of_matches(n: i32) -> i32 {
// //         let mut n = n;
// //         let mut result = 0;

// //         while n > 1 {
// //             if n % 2 == 0 {
// //                 result += n / 2;
// //                 n = n / 2;
// //             } else {
// //                 result += (n - 1) / 2;
// //                 n = (n - 1) / 2 + 1;
// //             }
// //         }

// //         result
// //     }
// // }

// // impl Solution {
// //     pub fn min_partitions(n: String) -> i32 {
// //         let mut result = 0;
// //         for v in n.chars() {
// //             if v as usize - '0' as usize > result {
// //                 result = v as usize - '0' as usize;
// //             }
// //         }
// //         result as i32
// //     }
// // }

// impl Solution {
//     pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
//         let n = stones.len();
//         let mut prefix: Vec<i32> = vec![0; n + 1];
//         let mut temp = 0;
//         for i in 0..n {
//             temp += stones[i];
//             prefix[i + 1] = temp;
//         }
//         let mut dp_m:HashMap<(usize,usize), i32> = HashMap::new();
//         Self::dp(0, n - 1, &mut dp_m,&mut prefix, &stones)
//     }

//     fn dp(l: usize, r: usize, dp_m:&mut HashMap<(usize,usize),i32>, prefix: &mut Vec<i32>, stones: &Vec<i32>,) -> i32 {
//         if l > r {
//             return 0;
//         } else {
//             if let Some(&res) = dp_m.get(&(l,r)){
//                 return res;
//             }else {
//             let sum = prefix[r + 1] - prefix[l];
//             let right_max = sum - stones[l] - Self::dp(l + 1, r, dp_m,prefix, stones);
//             let left_max = sum - stones[r] - Self::dp(l, r - 1, dp_m,prefix, stones);
//             dbg!(&right_max.max(left_max).clone());
//             return right_max.max(left_max).clone();
//         }
//     }
// }

// #[test]
// fn test() {
//     assert_eq!(Solution::stone_game_vii(vec![5, 3, 1, 4, 2]), 6);
//     assert_eq!(
//         Solution::stone_game_vii(vec![7, 90, 5, 1, 100, 10, 10, 2]),
//         122
//     );
// }
