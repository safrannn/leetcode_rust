struct Solution;
// use vec_string;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let mut fizz: bool;
        let mut buzz: bool;
        for i in 1..=n {
            fizz = i % 3 == 0;
            buzz = i % 5 == 0;
            let res = match (fizz, buzz) {
                (true, true) => "FizzBuzz".to_string(),
                (true, false) => "Fizz".to_string(),
                (false, true) => "Buzz".to_string(),
                (false, false) => format!("{}", i),
            };
            result.push(res);
        }
        result
    }
}

#[test]
fn test() {
    let output: Vec<String> = vec_string![
        "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14",
        "FizzBuzz"
    ];
    assert_eq!(Solution::fizz_buzz(15), output);
}
