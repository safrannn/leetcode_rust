struct Solution;

use rand::distributions::Uniform;
use rand::thread_rng;
use rand::Rng;

impl Solution {
    pub fn rand10() -> i32 {
        let mut i = 50;
        while i > 40 {
            i = (Self::rand7() - 1) * 7 + Self::rand7();
        }
        (i - 1) % 10 + 1
    }
    fn rand7() -> i32 {
        let distribution: Uniform<i32> = Uniform::new(0, 7);
        let mut rng = thread_rng();
        rng.sample(distribution) + 1
    }
}
