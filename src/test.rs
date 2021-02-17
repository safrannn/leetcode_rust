struct Solution;

impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut count = vec![0i32; 36];

        for v in low_limit..=high_limit {
            let mut num = v.clone();
            let mut sum = 0;
            while num > 0 {
                sum += num % 10;
                num /= 10;
            }
            count[sum as usize] += 1;
        }

        return *count.iter().max().unwrap();
    }
}

