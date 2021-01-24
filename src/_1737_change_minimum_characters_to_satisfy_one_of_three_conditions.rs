struct Solution;

impl Solution {
    pub fn min_characters(a: String, b: String) -> i32 {
        let n_a = a.len();
        let n_b = b.len();
        let a: Vec<char> = a.chars().collect();
        let b: Vec<char> = b.chars().collect();
        let mut a_count: Vec<usize> = vec![0; 26];
        for v in a {
            a_count[v as usize - 'a' as usize] += 1;
        }
        let mut b_count: Vec<usize> = vec![0; 26];
        for v in b {
            b_count[v as usize - 'a' as usize] += 1;
        }

        let mut a_leftsum: usize = 0;
        let mut b_leftsum: usize = 0;

        let mut result = std::i32::MAX;
        for i in 1..26 {
            let a_rightsum = n_a - a_leftsum;
            let b_rightsum = n_b - b_leftsum;
            result = result.min((a_leftsum + b_rightsum).min(a_rightsum + b_leftsum) as i32);

            a_leftsum += a_count[i];
            b_leftsum += b_count[i];
        }

        let a_single = a_count.iter().max().unwrap();
        let b_single = b_count.iter().max().unwrap();

        result = result.min((n_a + n_b - a_single - b_single) as i32);
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::min_characters("aba".to_string(), "caa".to_string()),
        2
    );
    assert_eq!(
        Solution::min_characters("dabadd".to_string(), "cda".to_string()),
        3
    );
}
