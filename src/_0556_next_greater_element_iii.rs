struct Solution;

impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut num: i32 = n.clone();
        let mut last: i32 = num % 10;
        let mut h_array: Vec<i32> = vec![last];
        num /= 10;

        while num > 0 {
            let mut current = num % 10;
            if last <= current {
                h_array.push(current);
            } else {
                let h_array_len: usize = h_array.len();
                let mut num_replace: i32 = h_array[h_array_len - 1];
                let mut index_replace: usize = h_array_len - 1;
                for i in 0..h_array_len {
                    if h_array[i] > current && h_array[i] <= num_replace {
                        num_replace = h_array[i].clone();
                        index_replace = i;
                        break;
                    }
                }
                h_array[index_replace] = current;
                current = num_replace;
                num = num / 10 * 10 + current;
                for v in h_array.iter() {
                    if num as i64 * 10 + *v as i64 > std::i32::MAX as i64 {
                        return -1;
                    } else {
                        num = num * 10 + v;
                    }
                }
                return num;
            }
            last = current;
            num /= 10;
        }
        -1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::next_greater_element(12), 21);
    assert_eq!(Solution::next_greater_element(21), -1);
    assert_eq!(Solution::next_greater_element(1999999999), -1);
}
