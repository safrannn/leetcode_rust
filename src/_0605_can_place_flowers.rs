struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut zero_count: i32 = 1;
        let mut count = 0;
        let mut flowerbed = flowerbed;
        flowerbed.push(0);
        flowerbed.push(1);
        print!("{:?}", flowerbed);

        for v in flowerbed {
            if v == 0 {
                zero_count += 1;
            } else {
                count += (zero_count - 1) / 2;
                zero_count = 0;
            }
        }
        count >= n
    }
}

#[test]
fn test() {
    assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
    assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2), true);
}
