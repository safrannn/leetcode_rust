struct Solution;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct LogEntry<'a> {
    sort_tag: i32,
    log: &'a str,
    id: &'a str,
}

impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let mut log_sort: Vec<LogEntry> = vec![];
        for (i, log) in logs.iter().enumerate() {
            let current_log: Vec<&str> = log.splitn(2, ' ').collect();
            let is_number: bool = {
                if current_log[1].chars().next().unwrap().is_ascii_digit() {
                    true
                } else {
                    false
                }
            };

            log_sort.push(LogEntry {
                sort_tag: if !is_number { -1 } else { i as i32 },
                log: current_log[1],
                id: current_log[0],
            });
        }
        log_sort.sort_unstable();

        let mut result: Vec<String> = vec![];
        for log in log_sort.iter() {
            result.push([log.id, " ", log.log].concat());
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::reorder_log_files(vec_string![
            "dig1 8 1 5 1",
            "let1 art can",
            "dig2 3 6",
            "let2 own kit dig",
            "let3 art zero"
        ]),
        vec_string![
            "let1 art can",
            "let3 art zero",
            "let2 own kit dig",
            "dig1 8 1 5 1",
            "dig2 3 6"
        ]
    );
}
