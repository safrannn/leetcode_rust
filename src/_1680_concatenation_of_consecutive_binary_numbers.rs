// struct Solution;

// impl Solution {
//     pub fn concatenated_binary(n: i32) -> i32 {
//         let mut result: usize = 0;
//         for i in 1..=n as usize {
//             result <<= Self::binary_digits(i);
//             result += i;
//             result %= 1_000_000_007;
//         }
//         result as i32
//     }

//     fn binary_digits(mut decimal: usize) -> usize {
//         let mut bits = 0;
//         while decimal > 0 {
//             bits += 1;
//             decimal >> 1;
//         }
//         bits
//     }
// }

// #[test]
// fn test() {
//     assert_eq!(Solution::concatenated_binary(1), 1);
//     assert_eq!(Solution::concatenated_binary(3), 3);
//     assert_eq!(Solution::concatenated_binary(12), 505379714);
// }
