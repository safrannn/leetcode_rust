struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left = 1;
        let mut right = 1000000001;
        while left < right {
            let mid = (left + right) / 2;
            if Self::can_finish(&piles, h, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }

    fn can_finish(piles: &Vec<i32>, h: i32, k: i32) -> bool {
        let mut hours = 0;

        for pile in piles {
            if pile % k == 0 {
                hours += pile / k;
            } else {
                hours += pile / k + 1;
            }
        }

        hours <= h
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
    assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
    assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
}
