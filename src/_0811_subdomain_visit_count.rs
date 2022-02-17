// struct Solution;
// use std::collections::HashMap;

// impl Solution {
//     pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
//         let mut map: HashMap<&str, i32> = HashMap::new();
//         for domain in cpdomains {
//             let domain_array: Vec<&str> = domain.split(" ").collect();
//             let occur: i32 = domain_array[0].parse().unwrap();
//             let d: Vec<&str> = domain_array[1].split(".").collect();
//             for v in d {
//                 let count = map.entry(v).or_default();
//                 *count += occur;
//             }
//         }
//         let mut result: Vec<String> = vec![];
//         for (domain, count) in map {
//             result.push(format!("{} {}", count, domain));
//         }
//         result
//     }
// }

// #[test]
// fn test() {
//     // assert_eq!(Solution::subdomain_visits(vec!["9001 discuss.leetcode.com".to_string()]), true);
// }
