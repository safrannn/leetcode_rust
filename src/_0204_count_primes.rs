struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 3 {
            return 0;
        }
        let mut is_prime = vec![true; n as usize];
        is_prime[0] = false;
        is_prime[1] = false;
        let mut i: usize = 2;
        while i * i < n as usize {
            if is_prime[i] {
                let mut j = i * 2;
                while j < n as usize {
                    is_prime[j] = false;
                    j += i;
                }
            }
            i += 1;
        }

        let mut count: i32 = 0;
        for v in is_prime {
            if v {
                count += 1;
            }
        }
        count
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_primes(345), 68);
    assert_eq!(Solution::count_primes(2), 0);
}
