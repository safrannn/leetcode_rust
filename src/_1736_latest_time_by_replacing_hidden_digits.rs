struct Solution;

impl Solution {
    pub fn maximum_time(time: String) -> String {
        let time = time.as_bytes();
        let mut first = time[0].clone();
        let mut second = time[1].clone();
        let mut third = time[3].clone();
        let mut fourth = time[4].clone();

        if first == '?' as u8 && second == '?' as u8 {
            first = '2' as u8;
            second = '3' as u8;
        } else {
            if first == '?' as u8 {
                if second as usize - '0' as usize >= 4 {
                    first = '1' as u8;
                } else {
                    first = '2' as u8;
                }
            }
            if second == '?' as u8 {
                if first == '2' as u8 {
                    second = '3' as u8;
                } else {
                    second = '9' as u8;
                }
            }
        }

        if third == '?' as u8 {
            third = '5' as u8;
        }

        if fourth == '?' as u8 {
            fourth = '9' as u8;
        }

        String::from_utf8(vec![first, second, ':' as u8, third, fourth]).unwrap()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::maximum_time("2?:?0".to_string()),
        "23:50".to_string()
    );
    assert_eq!(
        Solution::maximum_time("0?:3?".to_string()),
        "09:39".to_string()
    );
    assert_eq!(
        Solution::maximum_time("1?:22".to_string()),
        "19:22".to_string()
    );
}
