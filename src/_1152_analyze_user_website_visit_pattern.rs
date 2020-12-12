use std::collections::{HashMap, HashSet};
struct Solution;
impl Solution {
    pub fn most_visited_pattern(
        username: Vec<String>,
        timestamp: Vec<i32>,
        website: Vec<String>,
    ) -> Vec<String> {
        let mut data: HashMap<String, Vec<(i32, String)>> = HashMap::new();
        let n = username.len();
        for i in 0..n {
            data.entry(username[i].clone())
                .or_default()
                .push((timestamp[i], website[i].clone()));
        }

        let mut comb3: HashMap<(String, String, String), HashSet<String>> = HashMap::new();
        for (name, visit) in data.iter_mut() {
            let n_visit = visit.len();
            if n_visit < 3 {
                continue;
            }
            visit.sort();
            for i in 0..n_visit - 2 {
                for j in i + 1..n_visit - 1 {
                    for k in j + 1..n_visit {
                        comb3
                            .entry((visit[i].1.clone(), visit[j].1.clone(), visit[k].1.clone()))
                            .or_default()
                            .insert(name.to_string());
                    }
                }
            }
        }
        let mut max_ocur = 0;
        let mut result = ("".to_string(), "".to_string(), "".to_string());
        for (comb, ocur) in comb3.iter() {
            if ocur.len() > max_ocur {
                max_ocur = ocur.len();
                result = comb.clone();
            } else if ocur.len() == max_ocur && comb.clone() < result {
                result = comb.clone();
            }
        }
        vec![result.0.clone(), result.1.clone(), result.2.clone()]
    }
}

#[test]
fn test() {
    let username = vec_string![
        "joe", "joe", "joe", "james", "james", "james", "james", "mary", "mary", "mary"
    ];
    let timestamp = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let website = vec_string![
        "home", "about", "career", "home", "cart", "maps", "home", "home", "about", "career"
    ];
    let res = vec_string!["home", "about", "career"];
    assert_eq!(
        Solution::most_visited_pattern(username, timestamp, website),
        res
    );
}
